#include "csv_net.hpp"

CsvNet::CsvNet() {
    // Construct and register two Linear submodules.
    fc1 = register_module("fc1", torch::nn::Linear(10, 32));
    fc2 = register_module("fc2", torch::nn::Linear(32, 32));
    fc3 = register_module("fc3", torch::nn::Linear(32, 3));
}

// Implement the Net's algorithm.
torch::Tensor CsvNet::forward(torch::Tensor x) {
    // Pass the input through each layer
    x = torch::relu(fc1->forward(x));  // No need to reshape
    x = torch::relu(fc2->forward(x));
    x = fc3->forward(x);
    return x;
}
