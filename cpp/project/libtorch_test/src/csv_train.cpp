#include "csvdataset.hpp"
#include "Timer.hpp"
#include "csv_net.hpp"

int train_csv() {
  Timer t;
  // Check if CUDA is available
  if (!torch::cuda::is_available()) {
    std::cerr << "CUDA is not available! Exiting..." << std::endl;
    return -1;
  }
  // Set device to CUDA
  torch::Device device(torch::kCUDA);

  // Create a new Net.
  auto net = std::make_shared<CsvNet>();
  net->to(device);

  auto dataset = CsvDataset::builder()
                  .ignore_head()
                  .set_target_column(10)
                  .set_file_path("DataScience_salaries_2024.csv")
                  .set_category_columns({1,2,3,5,7,9,10})
                  //.set_ignored_columns({0})
                  .build()
                  .map(torch::data::transforms::Stack<>());

  // Create a multi-threaded data loader for the MNIST dataset.
  auto data_loader = torch::data::make_data_loader(std::move(dataset), 100);

  // Instantiate an SGD optimization algorithm to update our Net's parameters.
  torch::optim::SGD optimizer(net->parameters(), /*lr=*/0.08);

  for (size_t epoch = 1; epoch <= 1000; ++epoch) {
    size_t batch_index = 0;
    // Iterate the data loader to yield batches from the dataset.
    for (auto& batch : *data_loader) {
      // Reset gradients.
      optimizer.zero_grad();
      // Execute the model on the input data.
      auto data = batch.data.to(device).view({-1,10});
      //std::cout<<"data size: "<<data.sizes()<<std::endl;
      auto target = batch.target.to(device);//.view({-1,10});
      //std::cout<<"target size: "<<target.sizes()<<std::endl;
      torch::Tensor prediction = net->forward(data);
      // Compute a loss value to judge the prediction of our model.
      torch::Tensor loss = torch::nn::functional::cross_entropy(prediction, target);
      // Compute gradients of the loss w.r.t. the parameters of our model.
      loss.backward();
      // Update the parameters based on the calculated gradients.
      optimizer.step();
      // Output the loss and checkpoint every 100 batches.
      if (++batch_index % 100 == 0) {
        std::cout << "Epoch: " << epoch << " | Batch: " << batch_index
                  << " | Loss: " << loss.item<float>() << std::endl;
        // Serialize your model periodically as a checkpoint.
        torch::save(net, "net.pt");
      }
    }
  }
  return 0;
}


