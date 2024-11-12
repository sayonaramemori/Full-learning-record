### Another Mihomo Kernel  
- [Github Page](https://github.com/MetaCubeX/mihomo/tree/v1.18.10)  
- [Release Page](https://github.com/MetaCubeX/mihomo/releases/tag/v1.18.10)  
- [wiki](https://wiki.metacubex.one/startup/service/)  

> For Windows, exe file is recommended and deb for Linux  


### Configuration  
1. Install the installer, with github speed up, eg. `curl -L https://ghp.ci/https://github.com/MetaCubeX/mihomo/releases/download/v1.18.10/mihomo-linux-amd64-compatible-go120-v1.18.10.deb -o mihomo.deb`  
2. Run `sudo apt install ./mihomo.deb` and then `cd /etc/mihomo`  
3. Open your windows Configuration Directory of Clash_Verge and copy the `clash-verge` and `Country.mmdb` to `/etc/mihomo`  
4. Test with `curl -i google.com --proxy http://127.0.0.1:[YOUR_HTTP(S)_PORT]`  

### Configure for WebUI  
> To select node with GUI.  
```shell
sudo cat << 'EOF' > /etc/mihomo/temp.yaml
# 配置 WEB UI 目录，使用 http://{{external-controller}}/ui 访问
external-ui: /var/mihomo/ui/
external-ui-name: ui-one
external-ui-url: "https://ghp.ci/https://github.com/MetaCubeX/metacubexd/archive/refs/heads/gh-pages.zip"
EOF
sudo cat /etc/mihomo/config.yaml >> /etc/mihomo/temp.yaml
mv /etc/mihomo/temp.yaml /etc/mihomo/config.yaml
```




