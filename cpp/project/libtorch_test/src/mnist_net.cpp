#include "mnist_net.hpp"

Mnist::Mnist() {
    // Construct and register two Linear submodules.
    fc1 = register_module("fc1", torch::nn::Linear(784, 256));
    fc2 = register_module("fc2", torch::nn::Linear(256, 64));
    fc3 = register_module("fc3", torch::nn::Linear(64, 32));
    fc4 = register_module("fc4", torch::nn::Linear(32, 10));
}

// Implement the Net's algorithm.
torch::Tensor Mnist::forward(torch::Tensor x) {
    // Pass the input through each layer
    x = torch::relu(fc1->forward(x));
    x = torch::relu(fc2->forward(x));
    x = torch::relu(fc3->forward(x));
    x = fc4->forward(x);
    return x;
}
