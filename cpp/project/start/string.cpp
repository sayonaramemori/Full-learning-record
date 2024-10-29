#include <cstdio>
namespace string_allocation
{
	void test(){
		const char* a = "Hello, world";
		const char* b = "Hello";
		printf("a is %p",a);
		printf("b is %p",b);
	}
}
