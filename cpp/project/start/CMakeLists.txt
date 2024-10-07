cmake_minimum_required(VERSION 3.30)
project(helloworld)

set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_STANDARD 17)

file(GLOB SRC ${PROJECT_SOURCE_DIR}/*.cpp)

add_executable(app ${SRC})