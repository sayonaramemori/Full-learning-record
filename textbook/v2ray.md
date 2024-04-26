### 1. Open the websites below  
1. [Get started](https://www.v2ray.com/chapter_00/install.html)
2. [Release dist](https://github.com/v2ray/dist)  
3. [Full lessons](https://github.com/v2fly/fhs-install-v2ray)  

### 2. Download the correct Release from the dist 
> For my system, OS is Ubuntu 20.04.6 LTS (GNU/Linux 5.4.0-166-generic x86_64), and you can use `lscpu` to detect your computer information, as for my choice:  

### 3. Get config from windows  
1. Open v2ray GUI client  
2. Select the alive nodes  
3. Export it as client configuration  


### 4. Put them to your remote computer  
> If you work on your local computer, skip this step.   
```
sftp user@hostname
put [YOUR_ZIP_FILE_NAME]
put config.json
bye
```

### 5. Unzip it and test whether it works on your computer  
```shell
# Unzip the file to directory v2ray
unzip [YOUR_ZIP_FILE_NAME] -d v2ray
# Go into this directory
cd v2ray
# Test whether the binary file works or not.
./v2ray
# If it not works, go back to the dist page and find the version that satisfys your computer.
```

### 6. Move files to their place  
> These are the files which we should move manually.  


> You can copy the below script into a single shell script, and run it with bash  
```shell
#!/bin/bash
# Make sure your work directory is v2ray, the directory we have just created.
sudo mkdir /usr/local/share/v2ray/ /usr/local/etc/v2ray/ /var/log/v2ray/ -p
# Copy your config.json to cover the original
sudo cp ../config.json ./config.json

sudo cp ./v2ray /usr/local/bin/v2ray
sudo cp ./config.json /usr/local/etc/v2ray/config.json  
sudo cp ./geoip.dat /usr/local/share/v2ray/geoip.dat  
sudo cp ./geosite.dat /usr/local/share/v2ray/geosite.dat  
sudo cp ./systemd/system/v2ray.service /etc/systemd/system/v2ray.service
sudo cp ./systemd/system/v2ray@.service /etc/systemd/system/v2ray@.service

sudo systemctl daemon-reload
```

### 7. Start v2ray and stop it  
```shell
# This variable is work only for the current terminal if you simply run it on your command line
# Your can use cmd 'export | grep http' to check whether it exists or not, or put it to profiles.
export http_proxy="socks5://127.0.0.1:10808" && export https_proxy=$http_proxy

# Then start v2ray  
sudo systemctl start v2ray.service  
sudo systemctl status v2ray.service

# Stop service  
sudo systemctl stop v2ray.service  
unset http_proxy
unset https_proxy
```
