### Another Mihomo Kernel  
- [Github Page](https://github.com/MetaCubeX/mihomo/tree/v1.18.10)  
- [Release Page](https://github.com/MetaCubeX/mihomo/releases/tag/v1.18.10)  
- [Wiki Page](https://wiki.metacubex.one/startup/service/)  


### Configuration in Ubuntu  
1. Download the installer, with github proxy.  
```shell
# Using Github proxy ,here https://ghp.ci
# My cpu architecture is x86_64. Choose your version.

curl -L https://ghp.ci/https://github.com/MetaCubeX/mihomo/releases/download/v1.18.10/mihomo-linux-amd64-compatible-go120-v1.18.10.deb -o mihomo.deb
```
2. Simply install it via apt  
```shell
sudo apt install ./mihomo.deb
```
3. Open your windows configuration directory of Clash_Verge and copy the `clash-verge.yaml` and `Country.mmdb` to `/etc/mihomo`  
```shell
# You could choose other elegant way to achieve file transfermation.
# I use sftp here
# cd YOUR_WINDOWS_CONFIG_DIR
sftp user@host
put clash-verge.yaml
put Country.mmdb
bye

# Back to Linux
cd /etc/mihomo
sudo cp ~/Country.mmdb .
sudo cp ~/clash-verge.yaml ./config.yaml
```
4. Do Test  
```
# Start mihomo
sudo systemctl start mihomo

# Port is set in config.yaml with configuration item -- port.
curl -i google.com --proxy http://127.0.0.1:[YOUR_HTTP(S)_PORT]

# It should return some information with status code 301 if it works well.
```

### Configure for Dashboard  
> To select node freely with web-GUI.  
```shell
# Caution: Github proxy also used here
# Insert the three lines into config.yaml

sudo cat << 'EOF' > /etc/mihomo/temp.yaml
external-ui: /var/mihomo/ui/
external-ui-name: ui-one
external-ui-url: "https://ghp.ci/https://github.com/MetaCubeX/metacubexd/archive/refs/heads/gh-pages.zip"
EOF
sudo cat /etc/mihomo/config.yaml >> /etc/mihomo/temp.yaml
sudo mv /etc/mihomo/temp.yaml /etc/mihomo/config.yaml

# Reload config
sudo systemctl restart mihomo

# Access via http://{{external-controller}}/ui in browser
# The IP of your host is needed for accessment.
# Do remember set secret for security.
```

> Test whether GUI works well  
```
# My port is 7899 here 
# Create a test script

sudo cat << 'EOF' > /etc/mihomo/test.sh
#!/bin/bash
curl -i google.com --proxy http://127.0.0.1:7899
curl -i youtube.com --proxy http://127.0.0.1:7899
curl -i baidu.com --proxy http://127.0.0.1:7899
journalctl -u mihomo | tail
EOF

cd /etc/mihomo
sudo chmod o+x test.sh  

# You should see some information of the node you just have selected
./test.sh
```




