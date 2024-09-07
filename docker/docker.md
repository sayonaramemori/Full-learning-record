### Basic usage  
```shell
docker version
sudo docker info
docker search mysql
docker command --help
```

### How docker works  
- Docker is a Client-Server System, Deamon running. Access by Socket.  

### Quick start  
```shell
// create a container and then run it with specified container name
// -d run deamon 
// --name [container-name]
// -e required environment variable  
// -p host machine port : container port
// mysql:tag image_name:tag [image]
// export HTTP_PROXY=socks://localhost:9000 && HTTPS_PROXY=$HTTP_PROXY 

docker run -d --name someSql -p 1314:3306 -e TZ=Asia/shanghai -e MYSQL_ROOT_PASSWORD=123 mysql:latest
```

### Image Command  
```shell
//To pull a image from offical hub to local repository
docker pull [image:tag]

//To list the local image, -q list ID only
docker images [-q]

//To remove a specific local image, -f force to delete
docker rmi -f [image-id|name]

# Remove all images
docker rmi -f $(docker images -aq)
```

### Container Command
```shell
// list all container   
docker ps -a [-q]

// Start and go into the container  
docker run -it [container] [bash]

// Like linux top cmd
docker top [container-id]

// Inspect the information 
docker inspect [container-id] | grep Source

// Operation on Container  
docker stop/start/rm/restart/kill [container]

// Into the container by bash or sh
docker exec -it [container-id] [bash|sh]

// Follow the logs from this container
// -t   with timestamp  
// --tail number    
docker logs -f [container]

// Remove a container, -f force to delete  
docker rm [-f] [container]

// Copy file to host  
docker cp [container-id]:[path] [host-path]
```

### Volume Mount   
```shell
docker volume ls
docker volume inspect [volume-name]

#mount with name
docker run -v [name]:/var/lib/mysql --name mysql -d mysql

#anonymous mount
docker run -v /var/lib/mysql --name mysql -d mysql

#mount with a specific path 
docker run -v [path]:/var/lib/mysql --name mysql -d mysql
```

### World  
> Repository(remote) <---> `docker pull/push [image-name]` <--> local repository

> docker save/load [file] 
    - create a local file from repository
        - docker save -o file.tar [image]
    - load a tar file

> docker stats 
    - Show the CPU usage  

### Commit images(Local repository)  
- docker commit -m="description" -a="author" id name:tag

### Docker File  
```shell

```

### Network  
> Self define network: Every application in the same network can access every one by container name  
```shell
docker network --help  

// list network  
docker network ls  

// inspect a network
docker network inspect [net-name]

// check ip
docker exec -it [container] ip addr

// create a network
docker network create [--subnet 192.168.31.0/16] [--gateway 192.168.31.1] [network-name]

docker exec -it [container] ping [container-name]

// let a container join a new network
docker network connect [network-name] [container]

// disconnet  
docker network disconnect [network-name] [container]
```

### Example for v2  
1. pull from remote repository  
2. run with specified ports  
3. configure firewall for your computer  
4. export http\_proxy in terminal


