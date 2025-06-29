import deepxde as dde
import numpy as np
import torch

torch.set_default_dtype(torch.float64)
dde.config.set_default_float("float64")
dde.config.set_random_seed(666)
dde.model.optimizers.config.set_LBFGS_options(maxiter=20000);

Length = 1
# Define the domain
domain = dde.geometry.Rectangle([0, 0], [Length, Length])  # 2D domain (0, 1) x (0, 1)

# timedomain = dde.geometry.TimeDomain(0, 30)  # Time domain [0, 1]
# domain = dde.geometry.GeometryXTime(domain,timedomain)

def pde(x, y):
    # Extract the components of the output (velocity and pressure)
    u = y[:, 0:1]  # u-velocity component (x-direction)
    v = y[:, 1:2]  # v-velocity component (y-direction)
    
    # Compute the necessary derivatives
    u_x = dde.grad.jacobian(y, x, i=0,j=0)
    u_y = dde.grad.jacobian(y, x, i=0,j=1)
    v_x = dde.grad.jacobian(y, x, i=1,j=0)
    v_y = dde.grad.jacobian(y, x, i=1,j=1)

    # u_t = dde.grad.jacobian(y, x, i=0,j=2)
    # v_t = dde.grad.jacobian(y, x, i=1,j=2)
    
    # Pressure derivatives
    p_x = dde.grad.jacobian(y, x, i=2,j=0)  # Pressure gradient in x
    p_y = dde.grad.jacobian(y, x, i=2,j=1)  # Pressure gradient in y
    
    # Momentum equation components
    u_xx = dde.grad.hessian(u, x, i=0, j=0)  # u_xx (second derivative of u with respect to x)
    u_yy = dde.grad.hessian(u, x, i=1, j=1)  # u_yy (second derivative of u with respect to y)
    v_xx = dde.grad.hessian(v, x, i=0, j=0)  # v_xx (second derivative of v with respect to x)
    v_yy = dde.grad.hessian(v, x, i=1, j=1)  # v_yy (second derivative of v with respect to y)
    
    # Non-linear convection term
    conv_u = u * u_x + v * u_y  # u * u_x + v * u_y (nonlinear convection for u)
    conv_v = u * v_x + v * v_y  # u * v_x + v * v_y (nonlinear convection for v)
    # conv_u = 2 * u * u_x + v * u_y + u * v_y # u * u_x + v * u_y (nonlinear convection for u)
    # conv_v = u * v_x + 2 * v * v_y + v * u_x # u * v_x + v * v_y (nonlinear convection for v)
    
    # Incompressibility condition (continuity equation)
    continuity = u_x + v_y
    
    # Navier-Stokes equations for momentum balance
    ns_u = u_xx + u_yy  # Second derivatives of u for Laplacian term
    ns_v = v_xx + v_yy  # Second derivatives of v for Laplacian term
    # Re = v*L/nu ===> v = Re * nu / L = 1/L = 1/5
    ReynoldsNum = 1.0/500.0
    # Navier-Stokes PDEs for u and v components
    # eq_u = u_t + conv_u + p_x - ns_u/ReynoldsNum
    # eq_v = v_t + conv_v + p_y - ns_v/ReynoldsNum
    eq_u = conv_u + p_x - ReynoldsNum * ns_u
    eq_v = conv_v + p_y - ReynoldsNum * ns_v
    # eq_u = conv_u + p_x - 0.3 * ns_u
    # eq_v = conv_v + p_y - 0.3* ns_v
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
            res[i][0] = 1
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

bc_up_u = dde.icbc.DirichletBC(domain,lambda x: 1.0, boundary_up,component=0)
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
# net = dde.maps.FNN([2] + [50] * 4 + [3], "tanh", "Glorot normal")

# Create the model
data = dde.data.PDE(
# data = dde.data.TimePDE(
    domain,
    pde,
    [bc_up_u,bc_other_u,bc_v,bc_p_vert,bc_p_hori],
    num_domain=2560,
    num_boundary=1024,
    num_test=2560,
    # num_initial=1024
)
model = dde.Model(data, net)

# Compile the model
# model.compile("adam", lr=1e-3)
# model.compile("L-BFGS", lr=1e-3, metrics=["l2 relative error"])
model.compile("L-BFGS")

# Train the model
"""
checkpointer = dde.callbacks.ModelCheckpoint( "./model/top_cover_model.ckpt", verbose=1, save_better_only=True)
pde_resampler = dde.callbacks.PDEPointResampler(period=100)
losshistory, train_state = model.train(iterations=15000,display_every=500,callbacks=[checkpointer,pde_resampler])
dde.saveplot(losshistory, train_state, issave=True, isplot=True)
"""

# Prediction
model.restore("./model/top_cover_model.ckpt-59000.pt")
from visual_test import save_image,save_stream_line
x = np.linspace(0, Length, 150)  # X coordinates (adjust to your domain)
y = np.linspace(0, Length, 150)  # Y coordinates (adjust to your domain)
X, Y = np.meshgrid(x, y)  # Create a meshgrid for the 2D domain
x_flat = X.flatten()
y_flat = Y.flatten()
u = np.zeros_like(x_flat)  # Simulated pressure (replace with actual pressure field)
v = np.zeros_like(x_flat)  # Simulated pressure (replace with actual pressure field)
p = np.zeros_like(x_flat)  # Simulated pressure (replace with actual pressure field)
# for t in np.arange(0.0, 30, 5):
for i in range(u.__len__()):
    pre = model.predict([x_flat[i],y_flat[i]])
    u[i] = pre[0]
    v[i] = pre[1]
    p[i] = pre[2]

# Save the results as an image for the current time step
save_image(x, y, u, v, p, 99, filename="flow_output")
save_stream_line(x, y, u.reshape(150,150), v.reshape(150,150), p.reshape(150,150), 99, filename="flow_stream")
