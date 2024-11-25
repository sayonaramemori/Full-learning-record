### Install Redis  
- Using Docker  


### Configuration  
```
# for Docker Container
daemonize no
protected-mode no
# comment the default bind 127.0.0.1
requirepass YOURPASSWORD
databases [Number of Database]
```

### Start with Docker  

### Connect to redis  
- `redis-cli -a PASSWD -p 6379`


### Redis key  
```
keys * 
exists k1 k2 ...
type kn
del kn  
unlink kn       #async del
ttl kn          #Inspect the Time to Live
expire kn second  
move kn [dbindex]
select [dbindex]
dbsize          #Inspect the number of keys in current db
flushdb
flushall
```

### String  
```
//default infinite ttl
set key val [nx|xx] [get] [ex|px|exat|pxat|keepttl]
//nx not exist
//ex expire time length
//exat expire at timestamp
//keepttl when set ok

getrange kn 0 -1
getrange kn 0 end
setrange kn index [new-string]   //replace mode

//For number
incr kn [step]
decr kn [default 1]

strlen kn
append kn [new-string]

setex kn second val
setnx kn val
```

### List  
> Implemented be Dequeue  
```
lpush ln 1 2 3 4 5  
rpush ln 23 23 23
lrange ln 0 -1
lpop ln count
rpop ln count 

lindex ln [index]
llen ln
lrem ln [num] [val]  //del n items equal to val 
ltrim ln start end   //gain the substr and assign back to it 
rpoplpush origin dest //As its name
lset ln index val
linsert ln [before|after] [existing-val] [val-inserted]
```

### Hash  
```
hset hn:instance field1 val1 field2 val2
hget hn:instance field
hgetall hn:instance
hlen hn:instance  //number of fields
hexists hn:instance field
hkeys hn:instance
hvals hn:instance
hincrby hn:instance field step
hincrbyfloat hn:instance field float-step

//hsetnx hn:instance

```

### Set  
```
sadd key val
```

### Stream  
```

```

### Persistence  
 

### Master and Slave  
```

```

