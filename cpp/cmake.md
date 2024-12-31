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

set(SRC a.cpp b.cpp c.cpp)

add_executable(app ${SRC})
```

### File search  
> `file(GLOB\GLOB_RECURSE VAR PATH/*.cpp)`  
```cmake
file(GLOB_RECURSE SRC ${PATH}/*.cpp)
```

### Specify include directory  
```cmake
include_directories(${PROJECT_SOURCE_DIR}/include)
```

### Create Lib  
- Delete main function first  
```cmake
set(LIBRARY_OUTPUT_PATH /home/cpp/lib)
add_library(libName STATIC/SHARED SRC)
```

### Link a library  
```cmake
# add search path
link_directories(/home/lib /usr/lib)

# link static lib, use with add_executable
link_libraries(libName1 [libName2 libName3])
```

#### target_link_libraries  
- Link a static or shared lib  
- Link like a chains  
- `PUBLIC|PRIVATE|INTERFACE`  
- `target` can be an executable file or a lib  
```cmake
# A can access B and C
target_link_libraries(A B C)

# D can access B and C (public permission)
target_link_libraries(D A)

# Should be Located at last line for shared link
target_link_libraries(app libName)
```

### Add Macro  
```cmake
add_definitions(-DDEBUG)
```

### Nested CMakeLists  
> The CMakeLists can read the variable defined in top level cmakelists.  
```cmake
add_subdirectory(DIR)
```

### Link a Static lib in Static lib  

### Link a Shared lib in Static lib  
```cmake
add_library(lib STATIC ${SRC})
target_link_libraries(lib ${SHARED_LIB_NAME})
```

