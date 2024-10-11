#include <iostream>
#include <memory>
#include <cstring>

class String 
{
public:
    void print(){
        if(ptr!=nullptr)
            std::cout<<ptr<<std::endl;
        else 
            std::cout<<"nullptr"<<std::endl;
    }
    ~String(){
        std::cout<<"destroyed"<<std::endl;
        delete ptr;
    }
    String() = delete;
    String(const char* s){
        std::cout<<"constructed"<<std::endl;
        size = strlen(s) + 1;
        ptr = new char[size];
        memcpy(ptr,s,size);
    };
    String(const String& s){
        std::cout<<"Copyed"<<std::endl;
        this->size = s.size;
        this->ptr = new char[size];
        memcpy(ptr,s.ptr,size);
    };
    String(String&& s){
        std::cout<<"Moved"<<std::endl;
        this->size = s.size;
        this->ptr = s.ptr;
        s.size = 0;
        s.ptr = nullptr;
    }
    String& operator=(const String& s){
        if(&s == this){
            return *this;
        }
        std::cout<<"Assign"<<std::endl;
        delete this->ptr;
        this->size = s.size;
        this->ptr = new char[s.size];
        memcpy(ptr,s.ptr,size);
        return *this;
    }
    String& operator=(String&& s) noexcept{
        if(&s == this){
            return *this;
        }
        std::cout<<"Moved Assign"<<std::endl;
        delete this->ptr;
        this->ptr = s.ptr;
        this->size = s.size;
        s.ptr = nullptr;
        s.size = 0;
        return *this;
    }
private:
    size_t size;
    char *ptr;
};

int main()
{
    //constructor
    String a("java");
    a.print();
    //copy constructor
    String b = a;
    a.print();
    b.print();
    //move constructor
    String c = std::move(a);
    a.print();
    c.print();
    //assgin
    c = b;
    //move assgin
    c = std::move(b);
    c.print();
    b.print();
}