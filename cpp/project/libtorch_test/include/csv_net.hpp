#pragma once
#include <torch/torch.h>

struct CsvNet : torch::nn::Module {
  CsvNet();
  torch::Tensor forward(torch::Tensor x);
  // Use one of many "standard library" modules.
  torch::nn::Linear fc1{nullptr}, fc2{nullptr}, fc3{nullptr};
};

int train_csv();
