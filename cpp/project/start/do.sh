#!/bin/bash
rm -fr ./build
mkdir build && cd build
cmake .. && make
./app
