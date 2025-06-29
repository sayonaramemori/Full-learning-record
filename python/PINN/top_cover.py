import deepxde as dde
import numpy as np
import torch

torch.set_default_dtype(torch.float64)
dde.config.set_default_float("float64")
dde.config.set_random_seed(666)
dde.model.optimizers.config.set_LBFGS_options(maxiter=990000);

Length = 1.0
Vel = 1.0
Re = 100.0
# Define the domain
domain = dde.geometry.Rectangle([0, 0], [Length, Length])  # 2D domain (0, 1) x (0, 1)

# timedomain = dde.geometry.TimeDomain(0, 30)  # Time domain [0, 1]
# domain = dde.geometry.GeometryXTime(domain,timedomain)
def pde_gpt(x, u):
    u_vel = u[:, 0:1]
    v_vel = u[:, 1:2]
    p     = u[:, 2:3]

    # 一阶导
    u_x = dde.grad.jacobian(u_vel, x, i=0, j=0)
    u_y = dde.grad.jacobian(u_vel, x, i=0, j=1)
    v_x = dde.grad.jacobian(v_vel, x, i=0, j=0)
    v_y = dde.grad.jacobian(v_vel, x, i=0, j=1)
    p_x = dde.grad.jacobian(p,     x, i=0, j=0)
    p_y = dde.grad.jacobian(p,     x, i=0, j=1)

    # 二阶导
    u_xx = dde.grad.hessian(u_vel, x, component=0, i=0, j=0)
    u_yy = dde.grad.hessian(u_vel, x, component=0, i=1, j=1)
    v_xx = dde.grad.hessian(v_vel, x, component=0, i=0, j=0)
    v_yy = dde.grad.hessian(v_vel, x, component=0, i=1, j=1)

    continuity = u_x + v_y
    momentum_u = u_vel * u_x + v_vel * u_y + p_x - (1/Re) * (u_xx + u_yy)
    momentum_v = u_vel * v_x + v_vel * v_y + p_y - (1/Re) * (v_xx + v_yy)

    return continuity, momentum_u, momentum_v

def pde(x, y):
    u = y[:, 0:1]  # u-velocity component (x-direction)
    v = y[:, 1:2]  # v-velocity component (y-direction)
    
    u_x = dde.grad.jacobian(y, x, i=0,j=0)
    u_y = dde.grad.jacobian(y, x, i=0,j=1)
    v_x = dde.grad.jacobian(y, x, i=1,j=0)
    v_y = dde.grad.jacobian(y, x, i=1,j=1)
    p_x = dde.grad.jacobian(y, x, i=2,j=0)  # Pressure gradient in x
    p_y = dde.grad.jacobian(y, x, i=2,j=1)  # Pressure gradient in y
    
    u_xx = dde.grad.hessian(u, x, i=0, j=0)  # u_xx (second derivative of u with respect to x)
    u_yy = dde.grad.hessian(u, x, i=1, j=1)  # u_yy (second derivative of u with respect to y)
    v_xx = dde.grad.hessian(v, x, i=0, j=0)  # v_xx (second derivative of v with respect to x)
    v_yy = dde.grad.hessian(v, x, i=1, j=1)  # v_yy (second derivative of v with respect to y)
    
    conv_u = u * u_x + v * u_y  # u * u_x + v * u_y (nonlinear convection for u)
    conv_v = u * v_x + v * v_y  # u * v_x + v * v_y (nonlinear convection for v)
    
    continuity = u_x + v_y
    
    ns_u = u_xx + u_yy  # Second derivatives of u for Laplacian term
    ns_v = v_xx + v_yy  # Second derivatives of v for Laplacian term
    # Re = v*L/nu ===> v = Re * nu / L = 1/L = 1/5
    ReynoldsNum = 1.0/100.0
    # Navier-Stokes PDEs for u and v components
    # eq_u = u_t + conv_u + p_x - ns_u/ReynoldsNum
    # eq_v = v_t + conv_v + p_y - ns_v/ReynoldsNum
    eq_u = conv_u + p_x - ReynoldsNum * ns_u
    eq_v = conv_v + p_y - ReynoldsNum * ns_v
    return [continuity, eq_u, eq_v]

# Boundary conditions
# Input is x,y,t
def boundary_up(x, on_boundary):
    return on_boundary and dde.utils.isclose(x[1],Length)
def boundary_not_up(x, on_boundary):
    return on_boundary and not dde.utils.isclose(x[1],Length)

device = torch.device('cuda')
# Is this right?
def initial_condition(x):
    res = torch.DoubleTensor(1024,1)
    print(x.shape)
    for i in range(1024):
        if dde.utils.isclose(x[i][1],Length):
            res[i][0] = Vel
        else:
            res[i][0] = 0.0
    return res.to(device);

def pressure_bc_hori(x,on_boundary):
    return on_boundary and (dde.utils.isclose(x[1],0) or dde.utils.isclose(x[1],Length))
def pressure_bc_vert(x,on_boundary):
    return on_boundary and (dde.utils.isclose(x[0],0) or dde.utils.isclose(x[0],Length))
def robin_bc_vert(x,y,_):
    res = dde.grad.jacobian(y,x,i=2,j=0)
    return res
def robin_bc_hori(x,y,_):
    res = dde.grad.jacobian(y,x,i=2,j=1)
    return res

bc_up_u = dde.icbc.DirichletBC(domain,lambda x: Vel, boundary_up,component=0)
bc_other_u = dde.icbc.DirichletBC(domain,lambda x: 0.0, boundary_not_up,component=0)
bc_v= dde.icbc.DirichletBC(domain,lambda x: 0.0, lambda _,on_boundary: on_boundary,component=1)
bc_p_vert = dde.icbc.boundary_conditions.OperatorBC(domain,robin_bc_vert, pressure_bc_vert)
bc_p_hori = dde.icbc.boundary_conditions.OperatorBC(domain,robin_bc_hori, pressure_bc_hori)

ic_u= dde.icbc.IC(domain, initial_condition,lambda x, on_initial: on_initial ,component=0)
ic_v= dde.icbc.IC(domain, lambda x: 0,lambda x, on_initial: on_initial ,component=1)
# ic3= dde.icbc.IC(domain, lambda x: 0,lambda x, on_initial: on_initial ,component=2)

layer_size = [2] + [64] * 6 +  [3]
activation = "tanh"
initializer = "Glorot uniform"
net = dde.nn.pytorch.FNN(layer_size, activation, initializer)

# Create the model
data = dde.data.PDE(
# data = dde.data.TimePDE(
    domain,
    pde_gpt,
    [bc_up_u,bc_other_u,bc_v,bc_p_vert,bc_p_hori],
    num_domain=2560,
    num_boundary=256,
    num_test=2560,
    # num_initial=1024
)
model = dde.Model(data, net)

# Compile the model
# model.compile("adam", lr=1e-3)
# model.compile("L-BFGS", lr=1e-3, metrics=["l2 relative error"])
model.compile("L-BFGS")
model.restore("./model/top_cover_model.ckpt-59000.pt")

# Train the model
checkpointer = dde.callbacks.ModelCheckpoint( "./model/top_cover_model.ckpt", verbose=1, save_better_only=True)
pde_resampler = dde.callbacks.PDEPointResampler(period=100)
losshistory, train_state = model.train(callbacks=[checkpointer,pde_resampler])
dde.saveplot(losshistory, train_state, issave=True, isplot=True)
