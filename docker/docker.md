### Basic usage  
```shell
docker version
sudo docker info
docker search mysql
docker command --help
```

### Image Command  
```shell
docker pull [image:tag]
docker rmi -f [image-id|name]

# Remove all images
docker rmi -f $(docker images -aq)
```

### Container Command
```shell
docker top [container-id]
docker inspect [container-id] | grep Source
docker ps -a

docker stop/start/rm/restart/kill [container]

docker exec -it [container-id] [bash|sh]
docker logs -f [container]
```

### Mount Volume  
```shell
docker volume ls
docker volume inspect [volume-name]

#anonymous mount
docker run -v /var/lib/mysql --name mysql -d mysql

#mount with name
docker run -v [name]:/var/lib/mysql --name mysql -d mysql

#mount with path 
docker run -v [path]:/var/lib/mysql --name mysql -d mysql
```

### Quick start  
```shell
#create a container and then run it with specified container name
#-d run background
#--name [container]
#-e required environment variable  
#mysql:tag image_name:tag [image]
docker run -d --name someSql -p 1314:3306 -e TZ=Asia/shanghai -e MYSQL_ROOT_PASSWORD=123 mysql:latest
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

> docker stop/start/rm/restart/kill [container]

> docker logs/exec  
    - docker logs -f [container]
    - docker exec -it [container] bash


### ps  
> Show container available  
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
