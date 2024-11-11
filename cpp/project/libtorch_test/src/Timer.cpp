#include "Timer.hpp"
#include <chrono>
#include <iostream>

Timer::Timer()
{
    this->start = std::chrono::high_resolution_clock::now();
    std::cout<<"Start counting Time"<<std::endl;
}

Timer::~Timer()
{
    this->end= std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout<<"Cost "<< duration.count()<< "ms" <<std::endl;
}