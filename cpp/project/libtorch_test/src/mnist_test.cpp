#include "mnist_net.hpp"

int test_mnist(){
    // Load the trained model
    auto net = std::make_shared<Mnist>();
    torch::load(net, "net.pt");  // Load the saved model
    torch::Device device(torch::kCUDA);
    net->to(device);  // Move the model to GPU if using CUDA
    auto data_loader = torch::data::make_data_loader(
      torch::data::datasets::MNIST("./data/test",torch::data::datasets::MNIST::Mode::kTest).map( torch::data::transforms::Stack<>()), /*batch_size=*/100);
   
    net->eval();  // Set model to evaluation mode
  torch::NoGradGuard no_grad;  // Disable gradient computation during testing

  size_t correct = 0;
  size_t total = 0;

  for (const auto& batch : *data_loader) {
    auto data = batch.data.to(device).view({ -1,784 });
    auto targets = batch.target.to(device);

    // Forward pass (get predictions)
    auto output = net->forward(data);

    // Get predicted class
    auto prediction = output.argmax(1);
    correct += prediction.eq(targets).sum().item<int>();
    total += targets.size(0);
  }

  float accuracy = static_cast<float>(correct) / total * 100.0;
  std::cout << "Test Accuracy: " << accuracy << "%" << std::endl;
  return 0;
}
