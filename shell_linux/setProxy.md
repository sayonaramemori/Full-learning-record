### For apt  
```shell
sudo vim /etc/apt/apt.conf.d/01proxy

### Add these lines
Acquire::http::Proxy "http://127.0.0.1:10808/";
Acquire::https::Proxy "http://127.0.0.1:10808/";

### Simply disable the above to stop the proxy

sudo apt-get update
```

### For docker  
> Remove proxy by remove docker.service.d  
```shell
sudo mkdir -p /etc/systemd/system/docker.service.d
sudo vim /etc/systemd/system/docker.service.d/http-proxy.conf

### Add these 
[Service]
Environment="HTTP_PROXY=http://127.0.0.1:7899/"
Environment="HTTPS_PROXY=http://127.0.0.1:7899/"
Environment="NO_PROXY=localhost,127.0.0.1"

sudo systemctl daemon-reload
sudo systemctl restart docker

docker pull hello-world
```

