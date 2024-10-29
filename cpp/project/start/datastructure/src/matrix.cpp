#include <iostream>
#include <cstddef> 
#include <vector>
#include <algorithm>
#include <cstdlib>
#include "Timer.h"

using Shape = std::pair<int,int>;
using Vec = std::vector<double>;

class Matrix
{
    friend std::ostream& operator<<(std::ostream& os,const Matrix& v);
public:
    void transpose();
    Shape shape() const;
    double locate(size_t row,size_t col) const;
    Matrix(const Matrix&) = default;
    Matrix(size_t n);
    Matrix(std::vector<Vec>&&);
    Matrix(size_t row,size_t col,const std::vector<double> &ptr);
    Matrix operator*(const Matrix& rhv) const;
    bool operator==(const Matrix& rhv) const;
    Matrix operator*(double) const;
    Matrix& operator=(const Matrix& rhv) = default;
private:
    Matrix(const std::vector<Vec>&);
    int row;
    int col;
    std::vector<Vec> nums;
};

bool Matrix::operator==(const Matrix& rhv) const
{
    double delta = 1e-3;
    if((this->row == rhv.row) && (this->col == rhv.col)){
        for(int i=0;i<row;++i){
            for(int j=0;j<col;++j){
                auto temp = nums[i][j] - rhv.nums[i][j];
                if(temp>delta)return false;
            }
        }
        return true;
    }
    return false;
}

Matrix::Matrix(std::vector<Vec>&& v)
{
    std::cout<<"Moved constructor"<<std::endl;
    this->nums = std::move(v);
    this->row = nums.size();
    this->col = nums.size()==0?0:nums[0].size();
}

Matrix::Matrix(size_t n)
{
    row = col = n;
    nums = std::vector<Vec>(n,Vec(n));
}


// The numbers in ptr should >= r*c
Matrix::Matrix(size_t r, size_t c, const std::vector<double>& val)
{
    row = r;
    col = c;
    auto num = r*c;
    nums = std::vector<Vec>(r,Vec(c));
    for(int i=0;i<r;++i) std::copy_n(val.begin()+i*c,c,nums[i].begin());

}

Matrix Matrix::operator*(double k) const
{
    Matrix temp = *this;
    for(int i=0;i<row;++i){
        for(int j=0;j<col;++j){
            temp.nums[i][j] *= k;
        }
    }
    return temp;
}

Matrix Matrix::operator*(const Matrix& rhv) const
{
    if(this->col != rhv.row){
        std::cout<<"Multiply invalid, undefined behaviour"<<std::endl;
    }
    auto res = std::vector<Vec>(this->row,Vec(rhv.col));
    for(int r=0;r<this->row;++r){
        for(int c=0;c<rhv.col;++c){
            double temp = 0.0;
            for(int a=0;a<rhv.row;++a){
                temp += this->nums[r][a]*rhv.nums[a][c];
            }
            res[r][c] = temp;
        }
    }
    return Matrix(std::move(res));
}

void Matrix::transpose() { }

Shape Matrix::shape() const
{
    return std::make_pair(row,col);
}

// row and col start from zero  
double Matrix::locate(size_t r,size_t c) const
{
    return nums[r][c];
}

std::ostream& operator<<(std::ostream& os,const Matrix& v)
{
    os << v.row << " x " << v.col << "\n";
    for(int i = 0;i<v.row;++i)
    {
        for(int j=0; j<v.col;++j){
            os << "| " << v.locate(i,j) << " ";
        }
        os << "|\n";
    }
    return os;
}

int main()
{
    Timer t;
    // Vec a = {0,6,9,0.5,8,0,0,0.5,3};
    Vec a = {.8,.6,.3, 0.1,.2,.4, 0.1,.2,.3};
    Matrix b(3,3,a);
    const Matrix c(3,1,{18,3000,12});
    auto s = b*c;
    auto last = s;
    for(int i=0;i<40;++i){
        s = b*s;
        std::cout<<"iteration: "<<i<<std::endl;
        std::cout<<s*(1/s.locate(2,0))<<std::endl;
        if(s==last){
            auto v=s*(1/s.locate(2,0));
            std::cout<<"Converged"<<std::endl;
            std::cout<<b*v<<std::endl;
            break;
        }
        else last = s;
    }
}