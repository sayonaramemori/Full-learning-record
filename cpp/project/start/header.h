#pragma once
#include <string>
#include <iostream>

void rust();
void java();

class Entity
{
public:
	int age;
	std::string name;
	void print(){
		std::cout<<"Entity OK"<<std::endl;
	}
	Entity():age(0),name(""){ 
		std::cout<<"Entity created"<<std::endl;
	}
	Entity(int age_,std::string name_):age(age_),name(name_){
		std::cout<<"Entity created"<<std::endl;
	}
	~Entity(){
		std::cout<<"Entity Destroyed"<<std::endl;
	}
};
