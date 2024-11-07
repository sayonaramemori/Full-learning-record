### Comment  
```python
# It is a comment  
'''
multi-line comment 
'''
```

### Output format  
```python
age = 18  
# %.3f  %s
print('my age is %d',age)
print(f'my age is {age}')
```

### Numeric operation  
```python
a = 5
b = 2
print(a/b)  # Answer is float  
print(a//b) # 
c = a**b

```

### Control flow  
```python
# Compare operator: == != > < >= <=
if a>b:
    print('Ok')

# Logical operator: and or not 
if not a<b:
    print('No')
elif a==b:
    print('J')
else:
    print('L')

```

### Loop  
```python
while a<b:
    print('hh')

# [a,b)
for i in range(a,b):
    if i%2:
        # break
        continue
    print(i)
```

### str  
```python
str1 = 'java'
str2 = 'cpp'
# concat  
str3 = str1 + str2

# multiple
print(3 * str1)

# contain chars  
flag = 'ja' in str1
flag = 'ja' not in str1

# index  
print(str1[0])

# slice
print(str1[:])
print(str1[:2])
print(str1[1:])
print(str1[1:3])

# with step
# start:end:step
print(str1(-1:-5:-1))

# find substr return index(-1 for null)
str1.find('va')

str1.count('a')

str1.startswith('')
str1.endswith('')
str1.isupper()

# return a list  
str1.split(',')
```

### List  
> Can hold everything  
> Assign operator act as as a reference  
> Shallow copy and deep copy is considered.  

```python
s1 = 'ab ja jj kk ll'
l1 = s1.split(' ')

# append  
l1.append('java')

# insert 
l1.insert(index,value)

# modify a element  
l1[1] = 'sb'

# contains a element  
flag = 'sb' in l1
flag = 'sb' not in l1

# delete a specific element  
del l1[2]

# pop the last or specific index
li.pop()
li.pop(index)

# delete the list  
del l1

l1.sort()
l1.revese()

# candy  
[expression for i in list if condition]
```

### Tuple
> Only for query, no modification  
```python
tp = (1,'j',.99)
```

### Dictionary  
```python
dict1 = {'age':19, 'name': 'sb'}

# add a key
dict1['new'] = 'ls'

# delete a key-value pair  
del dict1['new']
del dict1

dict1.keys()
dict1.value()

for key,value in dict.items():
    print(f'{key} with value {value}')
```

### Set  
```python
s1={1,2,3}
s1.add(4)
# if not in, error raised
s1.remove(1)
# noexcept
s1.discard(2)

# 交集
s1 & s2
# 并集
s1 | s2
```

### Type convert  
```python
int('18')
float('19')
str(1)

# eval can convert str to list , dict and tuple 
str1 = '[1,2]'
li = eval(str1)

# list() can convert a iteratable 
list('abcdefg')
list((1,2,3,4))
# only keep keys
list(dict)
list({2,2,34,5})
```

### Shallow and Deep Copy  
```python
l1 = [1,2,3,[1,2,3]]
# Reference -- point to the same address
l2 = l1 

import copy

# copy address of the inner list
l3 = copy.copy(l1)

# copy all datas not address
l4 = copy.deepcopy(l1)
```

### Variable object
> No new allocation for modification  
> Include list, dict and set

> A new allocation happens when modifying  
> Invariable object: int, bool, float, complex, str, tuple

### Function and Args  
```python
def login():
    print('This is a function')
    # None is returned

def add(a,b):
    return a+b 

# a and b are forced to be provided when calling 
def funcs(a,b,*args):
    print(args)
    #args is a tuple
    print(type(args))

funcs(a,b,c,d,e)

def funck(**kwargs):
    #type is dict
    print(type(kwargs))

# key-value form when calling  
funck(name='java',age=18)
```

### Scope  
> Name-Lookup behaviours like cpp  
> Use `global` to declare a global variable in local scope  

### Lambda & Unpack  
```python
add = lambda a,b:a+b
l1 = [('cpp':12),('java':10)]
for i in sorted(l1,key=lambda a:a[1],reverse=True):
    print(i)

tp = (1,2,3)
# like rust but without brackets
a,b,c = tp
# b still is a tuple
a, *b = tp
```

### Error  
```python
def test():
    raise Exception('An error raised')
```

### Module & Package  
> A file with suffix `.py` is a module.  
> A directory with `__init__.py` is a package, and it will be executed when it was imported.     
> Import operation should be written in init file.  

> `[from module] import [module | class | variable | function | *] [as alias]`

> Use `__all__` to export  
```python
if __name__ == "__main__":
    print('Run')
```


### Class  
```python
class Person:
    # Constructor
    def __init__(self,age,name):
        self.age = age
        self.name = name
    # Deconstructor  
    def __del__(self):
        print('deleted')
    def printf(self):
        print(f"my age is {age}")

p1 = Person(19,'java')

# Add a new attribute for this instance  
p1.gender = 'male'

class Female(Person):
    # Placeholder
    pass
```
