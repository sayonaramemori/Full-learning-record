### Machine Learning  
- Looking for a function  

### Types of Functions  
1. Regression: The function outputs a scalar.  
2. Classification: Given options, the function outputs the correct one.  
3. Structure Learning  

### Loss function(Gradient descend)  
- To minimize the Loss  
- $L=\frac{1}{N} \sum_{i=0}^nE$
    - $g=\nabla L(w_0)$
    - $w_1 = w_0 - \eta\times g$

### Activation Functions  

#### Sigmoid Function  
- $y = b + \sum_{i}c_isigmoid(b_i+\sum_jw_{ij}x_j)$
- Provide the ability to simulate any function  

#### Rectified Linear Unit(ReLU) Function  
- $y = b + \sum_{2i}c_imax(0,b_i+\sum_jw_{ij}x_j)$

#### Find suitable function with the selected features  
![feature](./img/feature.jpg)  

#### Sum of series of Sigmoid function to achieve the truth function    
![neural](./img/neural.jpg)




