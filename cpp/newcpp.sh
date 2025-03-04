if [ -d $1 ];then
    echo "Directory $1 already exists"
    exit
else
    mkdir $1 -p
fi

cd $1
mkdir src inc lib -p
mkdir src/lib src/bin
cat << 'EOF' > CMakeLists.txt
cmake_minimum_required(VERSION 3.28)
project(MyProj)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_STANDARD 17)
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR})
file(GLOB SRC ${PROJECT_SOURCE_DIR}/src/lib/*.cpp)
file(GLOB MAIN ${PROJECT_SOURCE_DIR}/src/bin/main.cpp)
include_directories(${PROJECT_SOURCE_DIR}/inc)
link_libraries(${PROJECT_SOURCE_DIR}/lib)
add_executable(app ${MAIN} ${SRC})
#target_link_libraries(app B C)
EOF

cat << 'EOF' > ./src/bin/main.cpp
#include<iostream>
int main(){
    std::cout<<"Hello World"<<std::endl;
}
EOF

cat << 'EOF' > build.sh 
rm ./build -rf
cmake -B build -DCMAKE_BUILD_TYPE=Release -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
cp ./build/compile_commands.json .
cmake --build build --config Release
EOF

chmod +x build.sh
