cmake_minimum_required(VERSION 3.18 FATAL_ERROR)
project(deeplearning)

set(CMAKE_CUDA_ARCHITECTURES 89)
set(CMAKE_PREFIX_PATH /home/kazusa/libtorch_cuda/libtorch)
find_package(Torch REQUIRED)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${TORCH_CXX_FLAGS}")
file(GLOB SRC ${PROJECT_SOURCE_DIR}/src/*.cpp)
include_directories(${PROJECT_SOURCE_DIR}/include)

add_executable(app ${SRC})
target_link_libraries(app "${TORCH_LIBRARIES}")
set_property(TARGET app PROPERTY CXX_STANDARD 20)

