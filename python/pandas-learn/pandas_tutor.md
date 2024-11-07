#the ## Install pandas  
> run `pip install pandas`  

### File type  
|type|comment|method|  
|:--|:--|:--|  
|csv,tsv,txt|plain text splited by comma or tab| pd.read_csv|  
|excel|xls,xlsx|pd.read_excel|  
|mysql||pd.read_sql|  

### Data Structure  
#### DataFrame  
> A sheet -- two dimension    
```python 
#create a df with dict with sequence value
df = pd.dataFrame(dict:list)

#query a column, result is a series
df['state']
#query several columns, result is also a df
df[['state','year']]

#query a line , result is a series
df.loc[1]
#query several lines, result is also a df
df.loc[1:2]

df.set_index('name',inplace=True)
df.loc['index','column']
df.loc[[index],[column]]
#use : to match a range
df.loc[s:e,c_s:c_e]
#use fn -> bool to filter 

#add column
df[:,'new-column'] = df['c1'] - df['c2']
#use apply, apply a function along an axis of the df.
#x is a line , axis == 1
#x is a column , axis == 0
def get_temp(x):
    if x['colume-index'] :
        return 'high temp'
    else
        return 'low temp'

df.loc[:,'new col'] = df.apply(get_temp,axis=1)
#use assign
df.assign()
```

#### Series  
> A line or a column -- single dimension  
```python
#create a series
s1 = pd.Series(list)
s1.index
s1.values
#create a series with specified index
s2 = pd.Series([1,1,2],index=['a','b','c'])
#create a series with a dict
s = pd.Series(dict)
```

### Data wash  
```python
#isnull notnull
#dropna
    #axis 'index' or 'column'
    #how any or all
    #inplace
#fillna
    #value
    #method 'ffill' 'bfill'
    #axis and inplace
stu = pd.read_excel(path,skiprows=2)
stu.isnull()
stu['score'].isnull()

#del nan col
stu.dropna(axis='columns',how='all', inplace=True)
stu.dropna(axis='index',how='all', inplace=True)

#fill nan
stu.fillna({'score':0})
stu.loc[:,'score'] = stu['score'].fillna(0)

#fill forward
stu.loc[:,'name'] = stu['name'].fillna(method='ffill')

stu.to_excel('file',index=False)

#axis == 1 cross column, axis == 0 cross index
#del a col
stu.drop('a',axis=1)
#del a line
stu.drop(1,axis=0)
```

### Sorting  
```rust
Series.sort_values(asending=True,inplace=True)
DataFrame.sort_values(by,asending,inplace)
```

### str handle  
```python
#use .str to get series str attribution

#replace('old','new')
df['temp'].str.replace('C','')

#get condition
condition = df['temp'].str.start_with(',,')
df[condition]

#chain call
df['time'].str.replace('nain','').str.replace('yue','').str.replace('ri','')
```


### Index
```python
df.set_index('name',inplace,drop)
```


### Merge
```python
#left_on right_on  
pd.merge(left,right,how,on,...)

#how: inner outter left right
new_df = pd.merge(1,2,key,how='inner')

#relationship between sheet, use how to control the behaviour
    # 1 - 1  use on
    # 1 - N  data will be copyed
    # N - M  like a matrix mul
```  

![join](./join.jpg)

### Concat  
> Batch converge for excels with the same structure
```python
pd.concat(objs,axis,join,ingore_index)
#objs -- a list of df or series
#axis -- default 0, denoting append a line
#join -- default outer, inner

pd.concat([df1,df2])

#append a line
pd.append(other,ignore_index)

```

