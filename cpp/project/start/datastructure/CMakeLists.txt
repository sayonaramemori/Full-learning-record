cmake_minimum_required(VERSION 3.30)
project(helloworld)
file(GLOB_RECURSE SRC ${PROJECT_SOURCE_DIR}/src/*.cpp)
include_directories(${PROJECT_SOURCE_DIR}/include)

set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_STANDARD 17)

if(POLICY CMP0156)
  cmake_policy(SET CMP0156 NEW)
endif()
add_executable(app ${SRC})
