#include "csvdataset.hpp"
#include <optional>

#include <vector>
#include <set>
#include <initializer_list>

std::ostream& operator<<(std::ostream &os,const CsvDataset& cd){
    os << "Features:                                   "<< "Target\n";
    size_t size = *(cd.size());
    for(int j=0;j<size;++j){
        for(int i=0;i<cd.data[j].size();++i){
            os<<" | "<<cd.data[j][i];
        }
            os<<" | -------- " << cd.target[j] << "\n";
    }
    return os;
}

CsvDataset::CsvDataset()=default;

CsvDataset CsvDataset::builder(){
    return CsvDataset();
}

CsvDataset& CsvDataset::ignore_head(){
    this->ignore_headline = true;
    return *this;
}

CsvDataset& CsvDataset::set_target_column(int column){
    this->target_column = column;
    return *this;
}

CsvDataset& CsvDataset::set_file_path(const std::string &file_path){
    this->file_path = file_path;
    return *this;
}

CsvDataset& CsvDataset::set_category_columns(std::set<int> columns){
    this->category_columns= std::move(columns);
    return *this;
}

CsvDataset& CsvDataset::set_ignored_columns(std::set<int> columns){
    this->ignored_columns= std::move(columns);
    return *this;
}

// For the ignored, nullopt returned.
// For the category, index returned.
// For the normal, stof applied.
std::optional<float> CsvDataset::parse_item(int column,const std::string& val)
{
    if(ignored_columns.contains(column)){
        return std::nullopt;
    }else if(category_columns.contains(column)){
        auto res = get_category(val,column);
        if(res)return float(*res);
        else {
            std::cerr<<"Error while parsing data"<<std::endl;
            return std::nullopt;
        }
    }else {
        return std::stof(val);
    }
}

std::optional<int> CsvDataset::get_category(const std::string &value, int columns){
    int index = 0;
    for(const auto &v:this->specified_category[columns]){
        if(value == v)return { index };
        ++index;
    }
    return std::nullopt;
}

// The first line will be ignored
void CsvDataset::init_category(const std::string &file_path,const std::set<int> &list){
    std::ifstream file(file_path);
    std::string line;
    getline(file,line);
    int res = std::count_if(line.begin(),line.end(),[&](auto &i){return i == ','; });
    auto content = std::vector<std::set<std::string>>(res+1);
    while(std::getline(file,line))
    {
        std::stringstream ss(line);
        std::string value;
        int column_index = 0;
        // Parse each row
        while(std::getline(ss,value,','))
        {
            if(list.contains(column_index))content[column_index].insert(value);
            ++column_index;
        }
    }
    std::cout<<"Total column:  "<<content.size()<<std::endl;
    for(auto &v:content){
        std::cout<<"|"<<v.size();
    }
    std::cout<<"|"<<std::endl;
    this->specified_category = std::move(content);
}

CsvDataset CsvDataset::build()
{
    this->init_category(this->file_path,this->category_columns);
    this->load_data(this->file_path);
    return *this;
}

void CsvDataset::load_data(const std::string& file_path)
{
    std::ifstream file(file_path);
    std::string line;
    // Skip the first row
    if(this->ignore_headline)getline(file,line);
    while(std::getline(file,line))
    {
        std::stringstream ss(line);
        std::string value;
        std::vector<float> input_row;
        int column_index = 0;
        float target;
        // Parse each row
        while(std::getline(ss,value,','))
        {
            auto parsed_value = parse_item(column_index, value);
            if(!parsed_value){
                ++column_index;
                continue;
            }
            if(column_index == this->target_column)
            {
                target = *parsed_value;
            }else {
                input_row.push_back(*parsed_value);
            }
            ++column_index;
        }
        this->data.push_back(input_row);
        this->target.push_back(target);
    }
    this->num_features = data.empty()?0:data[0].size();
}

torch::data::Example<> CsvDataset::get(size_t index) {
    // Convert row into a tensor
    torch::Tensor tensor = torch::tensor(data[index]);
    // Reshape from (num,) to (1,num), -1 denote auto judge
    tensor = tensor.view({this->num_features});
    // Regression
    //torch::Tensor target_tensor = torch::tensor(target[index],torch::kFloat);
    // Classification
    //auto target_tensor = torch::zeros({3}, torch::kFloat);
    torch::Tensor target_tensor = torch::tensor(target[index],torch::kLong);
    //target_tensor[target[index]-1] = 1.0;
    return {tensor, target_tensor};
}

torch::optional<size_t> CsvDataset::size() const 
{
    return data.size();
}
