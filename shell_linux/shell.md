### run a shell 
> 只有第三种在当前进程直接执行，而第一二个则会开启一个新的进程来执行。
1. absolute path  
2. bash file.sh
3. source file.sh

### 单双引号
> 双引号可以变量引用，而单引号不行
```
name=cs
echo "$name" //cs
echo '$name' //$name
```

### string
> 可以由单双引号包裹，在无空格的情况下也可以不用
```
#1. lenght
str1=sbpz
echo ${#str1}

# = equal test
if [ $a = $b ]
# != not eqal
# -z is zero lenght
if [ -z $a ]
# -n is not empty
# test whether be blank
if [ $a ]
```

### array
```
myarr=('js' 'java' 'cpp')
1. get all
echo ${myarr[@]}
2. get by index
echo ${myarr[n]}
3. get length
len=${#myarr[@]}
```

### boolean

```
a=10
b=20
# -a -o ! || &&
if [ $a -lt 9 -a $b -gt 6 ]
then
else 
fi
```

### file test
```
file='path'
# can be read
if [ -r $file ]
#-w -x
# a normal file 
if [ -f $file ]
# a directory -d
# exists -e
```


### for loop
```
for val in array
do 
    cmd
done

for val in item1 item2 item3 ...
```
