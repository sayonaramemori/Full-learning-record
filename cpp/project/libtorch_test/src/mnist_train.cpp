#include "csvdataset.hpp"
#include "Timer.hpp"
#include "mnist_net.hpp"

int train_mnist() {
  Timer t;
  // Check if CUDA is available
  if (!torch::cuda::is_available()) {
    std::cerr << "CUDA is not available! Exiting..." << std::endl;
    return -1;
  }
  // Set device to CUDA
  torch::Device device(torch::kCUDA);

  // Create a new Net.
  auto net = std::make_shared<Mnist>();
  net->to(device);

  auto data_loader = torch::data::make_data_loader(
      //torch::data::datasets::MNIST("./data/test",torch::data::datasets::MNIST::Mode::kTest).map(
      torch::data::datasets::MNIST("./data/train").map(
          torch::data::transforms::Stack<>()),
      /*batch_size=*/100);

  // Instantiate an SGD optimization algorithm to update our Net's parameters.
  torch::optim::SGD optimizer(net->parameters(), /*lr=*/0.06);

  for (size_t epoch = 1; epoch <= 250; ++epoch) {
    size_t batch_index = 0;
    // Iterate the data loader to yield batches from the dataset.
    for (auto& batch : *data_loader) {
      // Reset gradients.
      optimizer.zero_grad();
      // Execute the model on the input data.
      auto data = batch.data.to(device).view({-1,784});
      // target is [100] not [100,1]
      auto target = batch.target.to(device);
      torch::Tensor prediction = net->forward(data);
      // Compute a loss value to judge the prediction of our model.
      torch::Tensor loss = torch::nn::functional::cross_entropy(prediction, target);
      // Compute gradients of the loss w.r.t. the parameters of our model.
      loss.backward();
      // Update the parameters based on the calculated gradients.
      optimizer.step();
      // Output the loss and checkpoint every 100 batches.
      if (++batch_index % 200 == 0) {
        std::cout << "Epoch: " << epoch << " | Batch: " << batch_index
                  << " | Loss: " << loss.item<float>() << std::endl;
        // Serialize your model periodically as a checkpoint.
          torch::save(net, "net.pt");
      }
    }
  }
  return 0;
}

