#pragma once  
#include <torch/torch.h>

struct Mnist : torch::nn::Module {
  Mnist();
  torch::Tensor forward(torch::Tensor x);
  // Use one of many "standard library" modules.
  torch::nn::Linear fc1{nullptr}, fc2{nullptr}, fc3{nullptr}, fc4{nullptr};
};

int train_mnist();
int test_mnist();

