#!/usr/bin/env python3
import os
import torch

os.environ["CUDA_VISIBLE_DEVICES"] = "0"

if torch.cuda.is_available():
    torch.set_default_device("cuda")
    print(">>> Using GPU:", torch.cuda.get_device_name(torch.cuda.current_device()))
else:
    print(">>> CUDA not available, falling back to CPU.")

import deepxde as dde
import numpy as np
import matplotlib.pyplot as plt

dde.config.backend = "pytorch"

geom = dde.geometry.Rectangle([0, 0], [1, 1])

def pde(x, u):
    u_vel = u[:, 0:1]
    v_vel = u[:, 1:2]
    p     = u[:, 2:3]

    u_x = dde.grad.jacobian(u_vel, x, i=0, j=0)
    u_y = dde.grad.jacobian(u_vel, x, i=0, j=1)
    v_x = dde.grad.jacobian(v_vel, x, i=0, j=0)
    v_y = dde.grad.jacobian(v_vel, x, i=0, j=1)
    p_x = dde.grad.jacobian(p,     x, i=0, j=0)
    p_y = dde.grad.jacobian(p,     x, i=0, j=1)

    u_xx = dde.grad.hessian(u_vel, x, component=0, i=0, j=0)
    u_yy = dde.grad.hessian(u_vel, x, component=0, i=1, j=1)
    v_xx = dde.grad.hessian(v_vel, x, component=0, i=0, j=0)
    v_yy = dde.grad.hessian(v_vel, x, component=0, i=1, j=1)

    Re = 100.0
    continuity = u_x + v_y
    momentum_u = u_vel * u_x + v_vel * u_y + p_x - (1/Re) * (u_xx + u_yy)
    momentum_v = u_vel * v_x + v_vel * v_y + p_y - (1/Re) * (v_xx + v_yy)

    return continuity, momentum_u, momentum_v

def boundary_bottom(x, on_boundary):
    return on_boundary and np.isclose(x[1], 0)

def boundary_top(x, on_boundary):
    return on_boundary and np.isclose(x[1], 1)

def boundary_sides(x, on_boundary):
    return on_boundary and (np.isclose(x[0], 0) or np.isclose(x[0], 1))

bc_b_u = dde.DirichletBC(geom, lambda x: torch.zeros((len(x), 1)), boundary_bottom, component=0)
bc_b_v = dde.DirichletBC(geom, lambda x: torch.zeros((len(x), 1)), boundary_bottom, component=1)
bc_s_u = dde.DirichletBC(geom, lambda x: torch.zeros((len(x), 1)), boundary_sides, component=0)
bc_s_v = dde.DirichletBC(geom, lambda x: torch.zeros((len(x), 1)), boundary_sides, component=1)
bc_t_u = dde.DirichletBC(geom, lambda x: torch.ones((len(x), 1)),  boundary_top, component=0)
bc_t_v = dde.DirichletBC(geom, lambda x: torch.zeros((len(x), 1)), boundary_top, component=1)

data = dde.data.PDE(
    geom,
    pde,
    [bc_b_u, bc_b_v, bc_s_u, bc_s_v, bc_t_u, bc_t_v],
    num_domain=4000,
    num_boundary=400,
)

layer_size = [2] + [64] * 6 +  [3]
net = dde.maps.FNN(layer_size, "tanh", "Glorot normal")

model = dde.Model(data, net)

checkpointer = dde.callbacks.ModelCheckpoint( "./model/top_cover_model.ckpt", verbose=1, save_better_only=True)
model.compile("L-BFGS")
losshistory, train_state = model.train(callbacks=[checkpointer])  # L-BFGS 会一直迭代到收敛

xx = np.linspace(0, 1, 100)
yy = np.linspace(0, 1, 100)
X, Y = np.meshgrid(xx, yy)
XY = np.vstack([X.flatten(), Y.flatten()]).T
UVP = model.predict(XY)
U = UVP[:, 0].reshape(100, 100)
V = UVP[:, 1].reshape(100, 100)

plt.figure()
plt.streamplot(X, Y, U, V, density=1.5)
plt.title("DeepXDE + PyTorch (GPU) with L-BFGS: Lid-Driven Cavity (Re=100)")
plt.xlabel("x")
plt.ylabel("y")
plt.show()

