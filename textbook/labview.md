### Features  
- If a stream has a metric, then the displayer should have the same metric as well.  

### Data Type  
1. Numeric  
    - Default double in Front-end.  
    - Default 32bit signed int in Backend(const value)  

#### String  
- 字符串函数  
    1. 字符串长度, 输入包括字符串常量，一维字符串数组(output an array)，多维字符串数组，簇，簇数组  
    2. 连接字符串，输入包括字符串标量，一维字符串数组(join)，多维字符串数组，簇，簇数组。函数输入个数应和图形对应。  
    3. 截取字符串  
    4. 替换子字符串  
    5. 正则替换  
    6. 格式化日期/时间字符串  
    7. **电子表格字符串至数组转换**  
    8. 数组至电子表格字符串转换(to excel)  

#### Bool  
- 布尔函数  
    1. 与，输入包括布尔标量，布尔数组，数值(二进制处理)，整型数组  
    2. 或  
    3. 非  
    4. 数组元素操作(数组与或,all_true)

#### Array  
1. 数组大小  
2. 索引(read only)，多维数组则消耗对应维度的输入来索引一个输出，或者输出一个子维度  
3. 替换数组子集(write)  
4. 数组插入(insert before)
5. 数组删除  
6. 初始化数组


#### 簇(struct)  
- 分解和捆绑  

#### 波形数据  


#### 枚举与下拉列表  
1. 下拉列表可以表示无符号，有符号，单精度，双精度等各种数值。而枚举只能是无符号整数；
2. 下拉列表输出的值可以任意指定，而枚举从0，增量为1；  
3. 枚举输入端和输出端要一致。

#### Express  

#### 自定义控件  


### 运算类型  
- 加减乘除

### Control Flow  

#### while  
- 循环体外的输入仅一次，第二次输入当且仅当该循环重置(i=0)
- 输出同理
- IO包括循环隧道（val），索引(array)，移位寄存器(register)
    - Input 包括循环隧道，移位寄存器，索引
    - Output 包括隧道模式(val,索引,条件)以及移位寄存器。
- while 内部变量为局部变量，while重置时也随着重置


#### For  
- 需指定循环次数，循环次数is min(N,Len(输入索引))，类似for in  

### 条件结构(switch)
- Input can be bool, int, string, enumerate and struct.  
    - Bool input leads two branches.  
- The output should be the same.  
- Int input leads multiple branches.  
    - Use comma to express `or`.  
    - Default branch should be specified.  


### 事件结构 Event Struct
- Block for event.  
- Different IO is accepted  

#### 过滤事件结构  
- Preprocess for this event.  

### 顺序结构  
- Task in sequence.  

### 公式节点  
- 封装的数学计算  
- A short C expression in text.(only support float)  

### 属性节点和方法  

#### 属性节点(get_sub_node, member)  
- Similar to html attributes  
- 本VI --- DOM  
- Include  
    - General(OwnVI,Father,ClassName and ID) 
    - UI(Position,bounds)
    - Widget Control  
        1. Blinking  
        2. Value  
    - Special Attributes  

#### 调用节点(member function)  
- Method of VI  

### 子VI  


### Multiple Thread  
- Multiple Loops are running in parallel  
- Every stream path can be viewed as a independent thread.  

#### Local variable  
- In a Loop, the local variable would not be reset for each iteration.  

#### Global variable  
- Share datas between threads and VI.  















