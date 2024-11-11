#include <iostream>
#include <string>

#define LOG(x) std::cout<<x
namespace lrvalue {


    using std::string;

    void print(string& v){
        LOG("lvalue")<<v<<std::endl;
    }
    void print(string&& v){
        LOG("rvalue")<<v<<std::endl;
    }

    void test()
    {
        string first = "hello";
        string last = "world";
        string full = first + last;
        print(full);
        print(first + last);
    }

}
