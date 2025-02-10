cmake_minimum_required(VERSION 3.30)

project(MyProj)

set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_CXX_STANDARD 17)

include_directories(${PROJECT_SOURCE_DIR}/inc)

file(GLOB SRC ${PROJECT_SOURCE_DIR}/src/*.cpp)

file(GLOB MAIN ${PROJECT_SOURCE_DIR}/main.cpp)

set(LIBRARY_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/lib)

link_directories(${PROJECT_SOURCE_DIR}/lib)

#add_definitions(-DDEBUG)

# ----------------
# add_executable(app ${SRC} ${MAIN})

link_libraries(myLoadDll)

add_executable(app ${MAIN})
 # target_link_libraries(app Dianji AGController)

# ----------------

# add_definitions(-DDLL_EXPORTS)
# add_library(myLoadDll STATIC ${SRC})
#
# target_link_libraries(Dianji AGController)
# ----------------

