### 下载v2rayN
1. 官方中文文档，[click](https://www.v2ray.com/chapter_00/install.html)
2. 官方[github](https://github.com/v2fly/fhs-install-v2ray)


### via bash to install
1. 到官方Github下载`install-release.sh`
2. 执行sudo bash install-relase.sh
3. 到windows中选中服务节点并作为客户端导出。
4. 到/usr/local/etc/v2ray, 将刚才导出的config.json替换。
5. 启动systemctl status v2ray
6. `export http_proxy="socks5://127.0.0.1:10808" && export https_proxy=$http_proxy`
7. `curl -i google.com` to test whether config is good.
