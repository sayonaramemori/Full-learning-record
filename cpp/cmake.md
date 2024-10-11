### Basic CMD  
> Use `#` to comment.  
> Use blank space or semicolon to seperate values.  
1. `cmake_minimum_required` to specify version required.  
2. `project` to specify project name.  
3. `add_executable` to specify a executable file.  

```cmake
cmake_minimum_required(VERSION 3.0)

project(helloworld)

add_executable(app a.cpp b.cpp d.cpp)
```

### set variable  
> `set(var [values])`  
```cmake
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_STANDARD 17)

set(SRC a.cpp;b.cpp;c.cpp)

add_executable(app ${SRC})
```

### file search  
> `file(GLOB\GLOB_RECURSE VAR PATH/*.cpp)`  
```shell
file(GLOB_RECURSE SRC ${PATH}/*.cpp)
```

### Specify include directory  
```shell
include_directories(${PROJECT_SOURCE_DIR}/include)
```

### Create Lib  
> Delete main function first  
```
set(LIBRARY_OUTPUT_PATH /home/cpp/lib)
add_library(libName STATIC/SHARED SRC)
```

### Specify linking library  
```
# add search path
link_directories(/home/lib /usr/lib)
# link static lib 
link_libraries(libName)
# link dynamic and static lib, target can be lib or executable file 
target_link_libraries(target libName)

add_executable(app ${SRC})
# should be at last line
target_link_libraries(app libName)
```

### Add Macro  
```
add_definitions(-DDEBUG)
```

### Nested CMakeLists  
> The CMakeLists can read the variable defined in top level cmakelists.  
```
add_subdirectory(DIR)
```
