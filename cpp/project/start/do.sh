#!/bin/bash
rm -fr ./build
mkdir build && cd build
cmake -DCMAKE_TOOLCHAIN_FILE=toolchain.cmake .. && make
./app
