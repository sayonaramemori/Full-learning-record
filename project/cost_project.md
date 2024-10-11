### Mysql Settings  
```shell
# Using docker to get start. A volume should be created first.  

docker volume create cost_record  

docker run --name cost_record -v cost_record:/var/lib/mysql -e MYSQL_ROOT_PASSWORD=printlnkazusa.vip121234 -e TZ=Asia/Shanghai mysql:latest  

# Into it  
docker exec -it cost_record bash

mysql -uroot -p  

# Create a database  
creaete database cost_record  

use cost_record

create table cost_record  

# Insert some datas  
insert into money (date,category,amount,description) values ('2024-10-01','Snack',10.50,'Shopping in grocery');

```
