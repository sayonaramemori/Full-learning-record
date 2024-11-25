#include <iostream>
#include <string>
using std::string;
#define LOG(x) std::cout<<x
#include <>

namespace lrvalue {
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
