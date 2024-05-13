## Basic  
### Operator  
1. Put a semicolon at the end of a statement suppressing the output.  
2. `+ - * / \ ^`
3. `> < >= <= == ~=`  
4. `|| && ~`  


### format for num  
> The default is to display numbers that have decimal points with four decimal places.  
```matlab
format long/short/loose/compact; 
2*sin(1.4)  
```

### Built-in Help  
> Use help command to gain help  
```matlab  
help sin
```

### Constants  
|variable|value|  
|:--|:--|  
|pi|3.14159...|  
|i|$\sqrt {-1}$|  
|j|$\sqrt {-1}$|  
|inf|infinity|  
|NaN|Such as $0/0$|  

### Random Numbers  
```matlab
rand %generate a num in (0,1)
round(rand*10) %generate a int num in (0,10) 
randi([3,6])  %return a random ingteger in the inclusive range.
```

### Built-in numerical functions  
> `help elfun`  
```matlab
round(pi,3)  %round to a specified number of digits  
nthroot(64,3) %the converse of ^ operator, 64^(1/3)
log(x)      %returns the natural logarithm  
log2(x)     %returns the base 2 logarithm  
log10(x)    %returns the base 10 logarithm  
exp(n)      %returns the constant e^n  
deg2rad(180) %output 3.1416  

```

## Vectors and Matrices  

### Creating Row Vectors  
```matlab 
v=[1 2 3 4] %saperators can be commas as well  
vec=2:6     %inclusive end
nv=1:2:9    %b:step:e
rnv=9:-2:1
ls=linspace(3,15,5) %(b,end,length)
logspace(1,4,4)     %10^1 10^2 10^3 10^4  
newvec=[nv ls]      %concat vectors in a line  
```

### Index for row vectors
```
newvec(5)  %index start from 1  
b=newvec(4:6)
c=newvec([1 10 5])
b(2)=99
rv=[2 4 11]
rv(4)=2     %The vector is extended to have four elements  
```

### Creating Column Vectors and Matrices
```matlab 
c=[1;2;3;4]
r=1:3;
c=r';  %create by transposing  
mat=[4 3 1;2 5 6]  %There must always be the same number of values in each column of a matrix.  
mat=[2:4;3:5]
rand(2) %generate 2x2 random matrix
rand(1,3) %generate 1x3 random matrix
randi([5,10],2) %generate 2x2 random integers
randi([5,10],2,3) %generate 2x3 random integers
zeros(n) %generate nxn 0 matrix  
ones(n,m)  %generete nxm 1 matrix  
```

### Index for matrix  
```matlab
mat=[2:4;3:5]

%indexing
mat(2,3)
mat(1:2,2:3)
mat(1,:)
mat(:,2)
mat(4) %linear indexing, matlat unwinds matrix column by column

%assign  
mat(2,:)=5:7;
mat(1:2,2:3)=zeros(2);  %or use a scalar —— a single 0
mat(1:2,2:3)=1
mat(:,4)=[9 2]'
```

### Dimensions  
> `length` and `size`  
```matlab
vec=-2:1
length(vec)  %its bigest number of rows or the columns
size(vec)    %[row col]
numel(vec)        %element numbers
mat(end,1)        %the element located in (end of the row,1) 
mat(1,end)        %the element located in (1,end of the column)
```

### Changing Dimensions  
```matlab
reshape(mat,n,m)
fliplr(mat) %flip from left to right,mirror vertically
flipud(mat) %flip from up to down, mirror horizontally
rot90(mat)  %rotate the matrix counterclockwise  
rot90(mat,n)%rotate by 90xn, n can be negative
repmat(mat,m,n) %copy mat and place it in mxn
vec=4:8
vec(3)=[]   %use [] to del elements
%if linear indexing is used with a matrix to delete an element, the matrix will be reshaped into a row vector  
mat=[7:9;4:6]
mat[5]=[]  %mat is a row vector now
```

### Vecctors and Matrices as function arguments  
```matlab
vec=-2:1;
absv=abs(vec);
signv=sign(vec);
min(vec);
max(vec);
sum(vec);
prod(vec); %return the product of all of the elements in a vector, for vec, it will return (-2)*(-1)*(0)*1. For matrix, it applys to columns.
diff(vec);
```

### Scalar and array operations on vectors and matrices  
```matlab
v=[3 7 8 9]
v=v*3   %multiply v by 3 to each element
v=v/2
v1=2:5
v2=[7:10]
v1+v2   
v1-v2
%operation that is based on multiplication, a dot must be placed firstly.
v.^2    %each element multiplying each element by itself
v1.*v2
```

### Matrix multiplication  
```matlab
A=[1,2,3;4,5,6];
B=[1:4;5:8:12:15];
A*B  %Simply remove the dot
```

## Introduction to matlab programming  

### Input Function  
```matlab
rad=input('Enter the radius:')
```

### Output Statements:disp and fprintf  
```matlab 
disp(variable)
fprintf('The result is %d\n',4^3)
```

### The plot function  
```matlab
%create coordinate variables and plot a red '*'
x=11;
y=48;
plot(x,y,'r*');

%change the axes and label them  
axis([9 12 35 55]);
xlabel('Time');
ylabel('Temperature');
title('Time and Temp');

%option for customizing a plot: color, line types and marker types
%color: blue green red cyan magenta yellow black(k) white
%marker: point(.) circle(o) x-mark(x) plus(+) star(*) square(s) diamond(d) down triangle(v) up triangle(^) left triangle(<) right triangle(>) pentagram(p) hexagram(h)
%line types: solid(-) dotted(:) dash dot(-.) dashed(-)
```

### Simple Related Plot Functions  
1. clf:clears the Figure Windows by removing everything from it.  
2. figure: create a new, empty Figure windows. Calling it as figure(n).  
3. hold:plots will be superimposed on the current one. hold on and hold off.  
4. legend:displays strings in a legend box in the figure window, in order of the plots in the Figure window  
5. displays grid lines on graph. on and off

```matlab
% This creates 2 different plots, in 2 different
% Figure Windows, to demonstrate some plot features
clf
x = 1:5; % Not necessary
y1 = [2 11 6 9 3];
y2 = [4 5 8 6 2];
% Put a bar chart in Figure 1
figure(1)
bar(x,y1)
% Put plots using different y values on one plot
% with a legend
figure(2)
plot(x,y1,'k')
hold on
plot(x,y2,'ko')
grid on
legend('y1','y2')]]
```

### Function Definition  
```matlab
function outputargument=fuctionname(input arguments)
end

function area = calcarea(rad)  
area = pi * rad * rad;
end

%call it
calcarea(4)
```

### Local Functions  
```matlab
x = 33;
y = 11;
a = locfn(x);
fprintf('a is %d\n', a)
fprintf('x is %d\n', x)

function out = locfn(in)
x = in + 5;
out = x;
end
```

## Control Flow  
```matlab
%logical value conversion in numbers is the same as C
if condition
    action1
else
    action2
end

for loopvar=range
    action  
end

[r, c] = size(mat);
for col = 1:c
    for row = 1:r
        % do something with mat(row,col)
    end
end

while condition
    action
end
```
