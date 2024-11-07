### Declare a variable  
> `nil` to `None`  
```
-- Default Global
b = 1
local a = 1
-- c is nil
a,b,c = 1,2
```

### Operator  
```lua
a = 1
b = 2
c = a*b
d = a^b
```

### String
> Like python  
```lua
a = 'java'
b = "rust"

-- Row string
c = [[fjakdjf
\\fdjkfdk
\t\\kfjdkjf]]

-- concat
c = a..b

-- to string
c = tostring(10)

-- to number, nil for failure
n = tonumber('9')

-- length of str
l = #str
```

### function  
```lua
function func(a,b)
    print(a,b)
    -- default
    return nil
    -- return multi-value
    -- return a,b
end

local i,j = f(1,2)
```

### table
```lua
-- index start from 1
a = {1, 'a', {'j'}}
print(a[2])
-- length  
print(#a)
-- insert
table.insert(a,"d")
table.insert(a,index,"d",)
-- remove
local tempo = table.remove(a,index)

-- index with str
a = {
    a = 1,
    b = 'java',
    [";,"] = 999
}
print(a.a)
print(a['a'])
print(a[';,'])
```

### Global Table  
> All global variable are stored in `_G`  
```lua
a = 1
print(_G['a'])
print(_G['table']['insert'])
```

### bool  
> False only when `false` or `nil`, 0 is `true`    
> 短路原则  
```lua
a = true
b = false
-- not equal
a ~= b
a and b
a or b
not a
```

### Control flow   
```lua
if 1>9 then
    print('a')
elseif 1<9 then
    print('c')
else
    print('b')
end
```

### loop
```lua  
-- [start end] with step  
-- i could not be modified  
for i=1,10,2 do
    print(i)
    if i==5 then break end
end

local n = 10
while n>1 do
    print(n)
    n = n-1
end
```

### require  
```lua  
-- Only load once
require("Filename")
-- Check whether loaded
package.loaded["Filename"]
-- unload
package.loaded["Filename"] = nil
```



