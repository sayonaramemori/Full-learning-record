#include "header.hpp"
#include <iostream>

class vbase{
    public:
    void info(){
        std::cout<<"Vbase"<<std::endl;
    }
    vbase(const vbase& v){
        std::cout<<"Copy Constructor"<<std::endl;
    }
    vbase(){
        std::cout<<"Constructor"<<std::endl;
    }
    virtual ~vbase(){
        std::cout<<"VBase des"<<std::endl;
    }
};

class derived: public vbase {
    public:
    virtual ~derived(){
        std::cout<<"Derive des"<<std::endl;
    }
};

int main()
{
    vbase a;
    auto b = new vbase(a);
    delete b;
    return 0;
}
