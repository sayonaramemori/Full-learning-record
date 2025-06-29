import deepxde as dde
import numpy as np
import torch

torch.set_default_dtype(torch.float64)
dde.config.set_default_float("float64")

def pde(x, y):
    dy_t = dde.grad.jacobian(y, x, i=0, j=1)
    dy_xx = dde.grad.hessian(y, x, i=0, j=0)
    d = 1
    
    return (
        dy_t
        - d * dy_xx
        - torch.exp(-x[:, 1:])
        * (3 * torch.sin(2 * x[:, 0:1]) / 2
           + 8 * torch.sin(3 * x[:, 0:1]) / 3
           + 15 * torch.sin(4 * x[:, 0:1]) / 4
           + 63 * torch.sin(8 * x[:, 0:1]) / 8
        )
    )
    


def func(x):
    return np.exp(-x[:, 1:]) * (
        np.sin(x[:, 0:1])
        + np.sin(2 * x[:, 0:1]) / 2
        + np.sin(3 * x[:, 0:1]) / 3
        + np.sin(4 * x[:, 0:1]) / 4
        + np.sin(8 * x[:, 0:1]) / 8
    )


geom = dde.geometry.Interval(-np.pi, np.pi)
timedomain = dde.geometry.TimeDomain(0, 1)
geomtime = dde.geometry.GeometryXTime(geom, timedomain)

data = dde.data.TimePDE(
    geomtime, pde, [], num_domain=320, solution=func, num_test=80000
)

dde.model.optimizers.config.set_LBFGS_options(maxiter=15000);

layer_size = [2] + [30] * 6 + [1]
activation = "tanh"
initializer = "Glorot uniform"
net = dde.nn.pytorch.FNN(layer_size, activation, initializer)

# Backend pytorch
def output_transform(x, y):
    return (
        x[:, 1:2] * (np.pi ** 2 - x[:, 0:1] ** 2) * y
        + torch.sin(x[:, 0:1])
        + torch.sin(2 * x[:, 0:1]) / 2
        + torch.sin(3 * x[:, 0:1]) / 3
        + torch.sin(4 * x[:, 0:1]) / 4
        + torch.sin(8 * x[:, 0:1]) / 8
   )

net.apply_output_transform(output_transform)

model = dde.Model(data, net)
# opt = torch.optim.LBFGS(net.parameters(),lr=1e-3,);

model.compile("L-BFGS", lr=1e-3, metrics=["l2 relative error"])
# model.compile("adam", lr=1e-3, metrics=["l2 relative error"])
# model.restore("my_model-5000.pt")
losshistory, train_state = model.train(iterations=10000)
# losshistory, train_state = model.train()
dde.saveplot(losshistory, train_state, issave=True, isplot=True)
# model.save("my_model")

