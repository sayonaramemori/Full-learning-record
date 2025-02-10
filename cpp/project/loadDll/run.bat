rm ./build/* -rf
cd build
cmake .. -G "MinGW Makefiles"
make
app.exe
cd ..



