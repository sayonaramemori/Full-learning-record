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
    const char* first = "hello";
    const char* last = "world";
    const char* java = "hello12";
    printf("%x,%x,%x",first,last,java);
  }

}
