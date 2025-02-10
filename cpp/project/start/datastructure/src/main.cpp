// #include "Timer.h"
#include <cstdio>
#include <future>
#include <chrono>
#include <thread>
#include <iostream>

int test_func(int a){
    printf("Received parameter is %d\n",a);
    return 0;
}

constexpr int fibonacci(int n) {
    return (n <= 1) ? n : fibonacci(n - 1) + fibonacci(n - 2);
}


int main(){
    // Timer a;
    std::future<int> result = std::async(std::launch::async,test_func,123);
    if (result.wait_for(std::chrono::milliseconds(100)) == std::future_status::ready) {
        std::cout << "Result: " << result.get() << std::endl;
    }else{
        std::cout << "Task did not complete within timeout." << std::endl;
    }
    constexpr int fib = fibonacci(10);  // 编译时计算
    printf("hello world %d\n",fib);
    return 0;
}
