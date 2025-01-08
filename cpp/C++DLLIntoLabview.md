### For C++ Interface  
1. Wrap the interface as C function-style  
2. Specify Marker on functions  
```cpp
#ifndef MYDLL_H
#define MYDLL_H

#ifdef DLL_EXPORTS
  // When building the DLL
  #define DLL __declspec(dllexport)
#else
  // When using the DLL
  #define DLL __declspec(dllimport)
#endif

#define CALL __stdcall
extern "C" {
    DLL double CALL add(double, double);
    DLL double CALL mul(double, double);
}
#endif
```
3. Implement in extern "C"  
```cpp
#include "header.h"
class MyClass{
    double a =0 ,b = 0;
public:
    double add(){
        return this->a + this->b;
    }
    double mul(){
        return this->a * this->b;
    }
    MyClass(double rhv,double lhv):a(rhv),b(lhv){}
};
extern "C" {
    DLL double CALL add(double a,double b){
        MyClass temp(a,b);
        return temp.add();
    }
    DLL double CALL mul(double a,double b){
        MyClass temp(a,b);
        return temp.mul();
    }
}
```

### Do test for dll  
> The dll should located in PATH or the executable file's path.  

### Import into Labview  
> If your Labview is 32 bit then your dll file should match it.  

### Your CMake  
```CMake
cmake_minimum_required(VERSION 3.30)
project(MyProj)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_STANDARD 17)
include_directories(${PROJECT_SOURCE_DIR}/include)
link_directories(${PROJECT_SOURCE_DIR}/lib)
file(GLOB SRC ${PROJECT_SOURCE_DIR}/src/*.cpp)
set(LIBRARY_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/lib)
add_definitions(-DDLL_EXPORTS)
add_library(Operators SHARED ${PROJECT_SOURCE_DIR}/src/lib.cpp)

# Generate a executable file
#add_executable(app ${PROJECT_SOURCE_DIR}/src/main.cpp)
# target_link_libraries(app libOperators)
```

### Your Build script  
```shell
rm build -rf
mkdir build
cd build
cmake -G "MinGW Makefiles" -DCMAKE_C_COMPILER=C:/MinGW/bin/mingw32-gcc.exe -DCMAKE_CXX_COMPILER=C:/MinGW/bin/mingw32-g++.exe ..
make 
cd ..
```
