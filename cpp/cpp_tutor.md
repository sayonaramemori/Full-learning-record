### Install by MSYS2 project  
1. [Click me to go](https://www.msys2.org/)  
2. Download it the and just follow its wizards  
3. `pacman -S make` to install make for cmake using.  
4. If your PC has installed VisualStudio, your should do:  

```shell
# open msys2 terminal
pacman -S make  

# Specify MinGW Makefiles generator in terminal  
cmake -G "MinGW Makefiles" ..

# Search make, may be it named with mingw32-make
```

### Head File Guard  
```c
// g++ support this feature
#pragma once

//or using macro  
#ifndef NOTATION_OF_THIS_FILE
#define NOTATION_OF_THIS_FILE
// Your code here
#endif
```

### Pre-Process for include and complie   
> Preprocess -> Compile -> Link  
> Simply paste them here  

> Compile independently then link the needed function entities compiled. So you can compile successful only with function declaration but fails when linking if no implementation.  

### Linking  
 > static(outsides of class) keyword denotea only visable inside its own file when linking.  
 
 > Link every necessay(used) unit COMPILED  

 > Standard develop flowsheet: Write declaration in head file then including them in main.cpp. The implementations can be anywhere.  
 
 ```mermaid
 flowchart LR
    A(Source Code) -->|Preprocess| B(Code with Pasted contents)
    B -->|Compile| C(Machine code unit)
    C -->|Link| D(Executable file or library)
 ```

### Variable in CPP  
> For bool 0 is false and !0 is true, occupys a byte.  
```cpp
unsigned int a = 99;
//print byte number  
std::cout << sizeof(bool) << std::endl;
```

### Function  
> Definition(in .cpp) and Declaration(in .h)  

```c++
// Head file guard to prevent duplication contain, g++ also supports this feature

#pragma once

//function signature
void a(int,int);
```

### Condition Statement  
```cpp
if (bool) {} else {}
```

### Loop  
```cpp
// combine with break and contine  
for(init;test;operation){}
while(bool){}
```

### Pointer  
```cpp
void *ptr = nullptr;
```

### Reference  
> It is a const pointer  
```cpp
int a = 5;
int& b = a;
```

### Class  
> Default private  

> static: shared by instance  

```cpp
class Player
{
    int x,y;
    //declaration
    static int speed;
    void move() {
        //this can be omitted
        this.x++;
        this.y++;
    }
    static void Move() {
        //only access static member
    }
}
//definition outside class declaration  
int Player::speed;
```

#### Constructor & Destructor  
```cpp
class Player
{
public:
    int x,y;
    Player(){
        x=0;
        y=0;
    }
    Player(int x,int y){
        this.x=x;
        this.y=y;
    }
    ~Player(int x,int y){
        //handle pointer here
    }
}

int main(){
    Player a(1,2);
}
```

### Constructor initializer list  
> Better performance.  
```cpp
class Entity {
public:
    std::string name;
    int score;
//first list then funciton body initialization
    Entity()
    : name("unknown"),score(12){}
    Entity(const std::string& name_):name(name_)
    {}
}
```


#### Class inheritence  
```cpp
class Gamer : public Player
{
    //add new features
}
```

#### Virtual function  
> To support polymorphism  
> Additional overhead  
```cpp
class Base {
public: 
    virtual string GetName() {return "BASE";}
}

class Derive{
public: 
    string GetName() override {return "Derive";}
}
int main() {
    Base a;
    Base* ptr_a = &a;
    Derive b;
    Base* ptr_b = &b;
    //if no virtual, all print "BASE"
    ptr_a->GetName();
    ptr_b->GetName();
}
```

#### Pure Virtual Function  
> Like a Interface, force to override  
```cpp
class Interface {
    virtual std::string GetName() = 0;
}
class Entity: public Base, Interface{
    //...
}
void PrintName(Interface* a){
    a->GetName();
}
```

#### Visiability  
```cpp
class Base
public:
    //visiable to all
protected:
    //only visiable to class and subclass
private:
    //only visiable inside. 
}
```

### Local static variable  
```cpp
void Funciton() {
    //set 0 when first encouter this line, with a program life time  
    static int i = 0;
}
```

### Enum  
> Represent a status with integer  
> Integer set  
```cpp
enum Status { 
    Good,
    Bad,
    Mediocre
} 
//no namespace
Status status = Good;
```

### Native Array  
```cpp
int example[10];
example[3] = 5;
int *ptr = example;
//equal to example[0]
*example = 10; 
//equal to example[9]
*(ptr + 9) = 10;

// Trick play
example[2] = 5;
*(int)((char*)ptr+8)=5;

//Calculate Size, only works on stack
int a[5];
int count = sizeof(a)/sizeof(int);
```


### string & std::string  
> A char array with \0 tail 
```cpp
const char* name = "Cherro";

// size is n + 1, for \0
char* myname = "Cherro";

//prohibit to modify literal,stored in readonly segment. 
myname[2] = 'J';  

// work the same, this can be modified  
char name_sqe[7]={'C','h','e','r','r','o',0};
name_seq[2] = 'J';  

std::string hatred = "java";
```

### const & mutable 
> `mutable` allows modification in a const method  
> `const` keyword before * is a constraint on the value.
```cpp
//const the value
type const* var;
const type* var;

//const the pointer
type* const var;

//Promise not to modify class 
//For const reference to call 
int GetVal() const{}
```

### Tri-operator  
```cpp
int a = 1?1:0;
```

### How to create a class  
```cpp
// Call default constructor  
Entity entity;

// With parameters  
Entity entity(paras);

// Create on heap  
Entity *e = new Entity();
delete e;
```

### Conversion, Implicit & Explicit  
1. (Bad Feature) Constructor receiving one parameter could implicit convert that to class using assignment operation   

2. Add `explicit` before the constructor to disable the feature.  

```cpp
class Entity
{
    public:
    int age;
    explicit Entity(int _age):age(_age){}
}

```

### Operator Overloading  
> Only use this feature for being truly meaningful
```cpp
// This function should be a friend 
// allow it to access private field  
std::ostream& operator<<(std::ostream& stream, const Entity& other){}

```


### Smart Pointer  
> Automatically new and delete, preventing us from memery leakage  
```cpp
#include <iostream>
#include <memory>

class MyClass {
public:
    MyClass() { std::cout << "MyClass constructed!" << std::endl; }
    ~MyClass() { std::cout << "MyClass destroyed!" << std::endl; }
    void greet() { std::cout << "Hello from MyClass!" << std::endl; }
};

int main() {
    // Create a unique_ptr to manage a MyClass instance
    std::unique_ptr<MyClass> ptr1 = std::make_unique<MyClass>();
    ptr1->greet();

    // Transfer ownership to another unique_ptr
    std::unique_ptr<MyClass> ptr2 = std::move(ptr1);

    if (!ptr1) {
        std::cout << "ptr1 is now null after ownership transfer." << std::endl;
    }

    // Use ptr2 to access MyClass
    ptr2->greet();

    // Resetting ptr2 (deletes the managed object)
    ptr2.reset();

    // At this point, MyClass has been destroyed, and ptr2 is now null
    if (!ptr2) {
        std::cout << "ptr2 is now null after reset." << std::endl;
    }

    return 0;
}
```

### Copy Constructor  
> A constructor receiving `const type&`  

> Default copy member(shallow copy)  
```cpp
// Define your copy constructor for deep copy  
```

### Arrow operator  
> Can be overloaded  
```cpp
// Get memory offset  
struct vectors{
    float x,y,z;
}
int main() 
{
    // Get z offset  
    int offset = (int)&((vectors*)nullptr)->z;
    std::cout<<offset<<std::endl;

    // Better Solution  
    #include <cstddef>
    std::cout << "Offset of a: " << offsetof(Matrix, row) << " bytes" << std::endl;
    std::cout << "Offset of b: " << offsetof(Matrix,col) << " bytes" << std::endl;
}
```

### Template  
> Templates are instantiated lazily  

### Macro  
```
#ifdef DEBUG
#define LOG(x) std::cout<< x <<std::endl;
#else
#define LOG(x)
#endif
```

### Function Pointer  
```cpp
void helloworld() 
{
    std::cout<<"hello world"<<std::endl;
}

int main()
{
    typedef void(*ILOVERUST)();
    ILOVERUST fn = helloworld;

    void (*cherro)() = helloworld;
    // Function pointer;
    // auto fn = &helloworld;
    auto fn = helloworld;
    fn();
    cherro();
}
```

### Lambda  
> Can be converted into fn pointer  
> `[capture mode](paras)->ret{body}`

### Thread  


### Chrono  
```cpp
#include <iostream>
#include <chrono>
#include <thread>
int main() 
{
    using namespace std::literals::chrono_literals;
    auto start = std::chrono::high_resolution_clock::now();
    std::this_thread::sleep_for(1s);
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<float> duration = end - start;
    std::cout<< duration.count << std::endl;
    std::cin.get();
}
```

### Multi-Dimension Array  
```cpp
int ** a2d = new int*[5];
int *** a3d = new int**[5];
```

### Virtual Destructor  
> To handle inheritence  
```
class Base
{
    Base(){}
    virtual ~Base(){}
};
class Derived: public Base
{
    Derive() {}
    ~Derive() {}
}

int main()
{
    Base* base = new Derived();
    // only ~Base called without virtual destructor.
    delete base;
}
```

### Access Control  
|Access Modifier in Base |Public Inheritance  |Protected Inheritance   |Private Inheritance|
|:--|:--|:--|:--|
|Public  |Public | Protected   |Private|
|Protected|   Protected|   Protected|   Private|
|Private |Inaccessible|    Inaccessible|    Inaccessible|

### Namespace  
> Class is also a namspace  
```cpp
namespace a
{
    void print();
}
int main() 
{
    //locally works
    using namespace a;
}
```

### Call Base method  
> Also apply on virtual method.  
```cpp
#include <iostream>
class Base {
    public:
    void show() const {
    std::cout << "Base class method\n";
                        }
};

class Derived : public Base {
    public:
    void show() const {
        // Call the base class method
        Base::show();
        std::cout << "Derived class method\n";
    }
};

int main() {
    Derived d;
    d.show(); // Calls both base and derived class methods
    return 0;
}
```

### C++ 17 New Features  

#### std::variant  


#### std::optional  
> Can be converted into bool.  
```cpp
#include <iostream>
#include <optional>
#include <string>
 
 // optional can be used as the return type of a factory that may fail
 std::optional<std::string> create(bool b)
 {
    if (b) return "Godzilla";
    return {};
 }

// std::nullopt can be used to create any (empty) std::optional
auto create2(bool b)
{
    return b ? std::optional<std::string>{"Godzilla"} : std::nullopt;
}

int main()
{
    std::cout << "create(false) returned " << create(false).value_or("empty") << '\n';
    // optional-returning factory functions are usable as conditions of while and if
    if (auto str = create2(true))
        std::cout << "create2(true) returned " << *str << '\n';
}
```

### lvalue & rvalue  
```
#include <iostream>
#include <string>
#define LOG(x) std::cout<<x
using std::string;

//Only receive lvalue
void print(string& v){
        LOG("lvalue")<<v<<std::endl;
}

//Only receive rvalue
void print(string&& v){
        LOG("rvalue")<<v<<std::endl;
}

int main()
{
        string first = "hello";
        string last = "world";
        string full = first + last;
        print(full);
        print(first + last);
}
```

### Operator new & delete  
> To inspect the memory allocation  
```cpp
void* operator new(size_t byte)
{
    std::cout<<"Allocation for "<<byte<<" byte"<<std::endl;
    return malloc(byte);
}
void operator delete(void* ptr,size_t byte)
{
    std::cout<<"Delete for "<<byte<<" byte"<<std::endl;
    free(ptr);
}
```

### Algorithm based on Iterator  
1. copy 
```cpp
// Copies the elements in the range defined by [first, last), 
// to another range beginning at d_first (copy destination range).
std::copy(first, last, d_first);
std::copy_n(first, count, d_fist);
```
2. fill
```cpp
// Assigns the given value to all elements in the range [first, last).
std::fill(first, last, const T& value)
std::fill_n(first, count, const T& value)
```

3. transform  
```cpp
// std::transform applies the given function to the elements of the given input range(s), 
// and stores the result in an output range starting from d_first.
std::transform(first,last,d_first,UnaryOp);
std::transform(first1,last1,first2,last2,d_first,BinaryOp)
```

4. find
```cpp
// Returns an iterator to the first element in the range [first, last) 
// that satisfies specific criteria (or last if there is no such iterator).
std::find(first,last,const T& value)
std::find_if(first,last,UnaryPred);
```

5. replace  
6. foreach  
```cpp
// Applies the given function object f 
// to the result of dereferencing every iterator 
// in the range [first, last). If f returns a result, the result is ignored.
for_each(first, last, UnaryFunc);
```

### Overload in inheritence  
1. If a virtual method, no influence  
2. If not a virtual method, one in the sub-class hides the all of the base.  

### Initialize class members  
> In C++, there are three common ways to initialize class members:  

1. Initializer List (in the constructor).  
    - Members are initialized directly, which is usually more efficient because it avoids a default initialization followed by assignment.  
    - Certain types of members (like const, references, and members of classes without a default constructor) must be initialized in the initializer list.  
2. Constructor Function Body.  
    - Members are first default-initialized (or zero-initialized if they are fundamental types), then they are assigned new values in the constructor body.  
    - This can lead to inefficient code because the member is initialized twice (first the default initialization, then the assignment).  
3. In-line Initialization (in the declaration of the member variable).  
    - This is useful when you want to provide a default value for a member, which is used unless the constructor provides an alternative initialization.  


|Method  |Description |Use Case    |Performance |Limitations|
|:--:|:--:|:--:|:--:|:--:|
|Initializer List    |Initializes members directly in the initializer list.   |Best for efficiency, required for const and references, and base classes.   |Most efficient, avoids double initialization.   |Required for const and references.|  
|Constructor Body    |Initializes members after they are default-initialized.| Use when initialization depends on complex logic.|   Can be less efficient (double initialization).|  Cannot be used for const and references. | 
|In-line Declaration |Initializes members at their point of declaration.  |Good for default values that don't change often.   | Efficient like initializer list for default values.| Overridden by constructor initialization.|  


### move constructor & move operator=  
> A shallow copy; constructor but take care of memory leakage.    
> `std::move` converts a lvalue to rvalue, to call move method.  
```
#include <iostream>
#include <memory>
#include <cstring>

class String 
{
public:
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
    String& operator=(String&& s){
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
//copy constructor
String b = a;
//move constructor
String c = std::move(a);
//assgin
c = b;
//move assgin
c = std::move(b);
}
```

### Async  

### Difference of Static and Shared Lib  

### 
