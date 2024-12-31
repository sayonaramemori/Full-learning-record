cmake_minimum_required(VERSION 3.30)

project(MyProj)

#set(CMAKE_CXX_STANDARD_REQUIRED True)
#set(CMAKE_CXX_STANDARD 17)

file(GLOB SRC ${PROJECT_SOURCE_DIR}/*.cpp)

link_libraries(${PROJECT_SOURCE_DIR}/lib)

# Generate a executable file
add_executable(app ${SRC})

#target_link_libraries(app B C)
