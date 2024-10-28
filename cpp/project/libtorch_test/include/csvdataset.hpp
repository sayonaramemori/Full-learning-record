#pragma once  
#include <torch/torch.h>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
#include <iostream>

class CsvDataset : public torch::data::Dataset<CsvDataset>
{
friend std::ostream& operator<<(std::ostream &os,const CsvDataset&);
private:
    size_t num_features;
    std::vector<std::set<std::string>> specified_category;
    std::set<int> category_columns;
    std::set<int> ignored_columns;
    bool ignore_headline = false;
    int target_column = 0;
    std::string file_path;
private:
    void load_data(const std::string &file_path);
    void init_category(const std::string &file_path, const std::set<int> &place);
    std::optional<int> get_category(const std::string &value, int columns);
    std::optional<float> parse_item(int column, const std::string&);
public:
    CsvDataset();
    static CsvDataset builder();
    CsvDataset& ignore_head();
    CsvDataset& set_target_column(int);
    CsvDataset& set_file_path(const std::string &file_path);
    CsvDataset& set_category_columns(std::set<int> columns);
    CsvDataset& set_ignored_columns(std::set<int> columns);
    CsvDataset build();
public:
    torch::data::Example<> get(size_t index);
    torch::optional<size_t> size() const;
private:
    std::vector<std::vector<float>> data;
    std::vector<float> target;
};

std::ostream& operator<<(std::ostream &os,const CsvDataset&);


