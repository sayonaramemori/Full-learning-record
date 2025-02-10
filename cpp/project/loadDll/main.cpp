#include "myheader.h"
#include "header.h"
#include <iostream>

typedef double (*Myadd)(double,double);

int main(){
    void* hmodule = load_dll("C:/Users/13427/Desktop/CppTest/lib/libOperators.dll");
    if(hmodule == NULL){
        std::cout<<"LOAD Failed"<<std::endl;
    }else{
        Myadd add_ptr = (Myadd)get_func(hmodule,"add");
        if(add_ptr==NULL){
            std::cout<<"Cannot find add"<<std::endl;
        }else{
            std::cout<<"Get Add"<<std::endl;
            std::cout<<"1+2="<<add_ptr(1,2)<<std::endl;
        }
        std::cout<<"LOAD OK"<<std::endl;
        free_dll(hmodule);
    }
}
