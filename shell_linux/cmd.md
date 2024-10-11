### User Manage  
```shell
# See all user info for last login
lastlog

# -m    create home directory if not exists  
# -s    specify shell  
useradd [name] -m -s /bin/bash

# change password for a user  
passwd [name]

# -r    remove home directory  
userdel -r [name]

# identify a user  
id [name]

# change current user
su - [name]

# who am i -- more detailed
whoami
```

### ln  
```shell
ln -s [origin] [link]
```

### tee  
> Read from stdin  
```shell
tee result.txt
```

### Process Substitution
```shell
# as a file-like input
diff <(ls dir1) <(ls dir2)

# as a file-like output
command1 | tee >(gzip > output.gz) | command2
```


### Run a shell script
> Only the third run in current shell environment.  
1. absolute path  
2. bash file.sh
3. source file.sh

### Process manage  
1. ps -aux | less
2. ps -ef | grep  
3. top
4. pstree  

### background task
```shell
cmd &
nohup cmd

# bring back to fore ground
fg

# list the bg job
jobs -l

# restart bg job
bg [num]
```

### File descriptor  
|descriptor|acronym|description|
|:----|:-----|:------|
|0|STDIN||
|1|STDOUT||
|2|STDERR||
```shell
'''Temporarily impose to output to descriptor, add & to >n
echo "This is err" >&2
ls 1> res.txt

'''force output to specified file, often used in shell script
'''stdout to outputfile and stderr to outputerr
exec 1>outputfile
exec 2>outputErr

'''create your won file descriptor
exec 3>outputfile
ls >&3
'''or append
exec 3>>outputfile

'''redirect descriptor
'''3 redirect ro 1
exec 3>&1

'''close descriptor, use &-
exec 3>&-

'''prevent output
ls -al > /dev/null
```

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
#1. length
str1=sbpz
echo ${#str1}

#2. = equal test
# != not eqal
if [ $a = $b ]

#3. -z is zero lenght
# -n is not empty
if [ -z $a ]

# test whether be blank
if [ $a ]
```

### chmod with find
```shell
# for directory
find [PATH] -type d -exec chmod 755 {} \;
find [PATH] -type d | xargs chmod 755

# for file
find [PATH] -type f -exec chmod 666 {} \;
find [PATH] -type f | xargs chmod 666 
```

### Time Zone  
```shell
timedatectl list-timezones
```
