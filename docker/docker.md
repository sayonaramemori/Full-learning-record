### Quick start  
```shell
#create a container and then run it with specified container name
#-d run background
#--name [container]
#-e required environment variable  
#mysql:tag image_name:tag [image]
docker run -d --name someSql -p 3306:3306 -e TZ=Asia/shanghai -e MYSQL_ROOT_PASSWORD=123 mysql:latest
```

### World  
> Repository(remote) <---> `docker pull/push [image-name]` <--> local repository

> docker images/rmi  
    - list images in local repository  
    - rm image from local repository

> docker save/load [file] 
    - create a local file from repository
        - docker save -o file.tar [image]
    - load a tar file

> docker stop/start/rm [container]

> docker logs/exec  
    - docker logs -f [container]
    - docker exec -it [container] bash


### ps  
```shell
#To list the current containers 
#-a all including exited containers
docker ps -a
```

### Mount data volume  
> To modify files in docker easily  
> docker volume --help  
```shell
#Maping directory
#-v [volume-name]:[Mounted Point]
#html is located in /var/lib/docker/volumes/
#[volume-name] start with / or ./, mount anywhere
docker run -d --name nginx -p 80:80 -v html:/usr/share/nginx/html nginx:latest
docker volume ls
docker volume inspect [volume-name]


#example  

```

### Docker File  
```shell

```

### Network  
> Self define network: Every application in the same network can access every one by container name  

```shell
docker network create [network-name]
#let a container join a new network
docker network connect [network-name] [container]
```

### Example for v2  
1. pull from remote repository  
2. run with specified ports  
3. configure firewall for your computer  
4. export http\_proxy in terminal
