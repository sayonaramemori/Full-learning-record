    systemctl restart v2ray
    sleep 3s
    HTTP_PROXY="socks5://127.0.0.1:10808/"
    HTTPS_PROXY=$HTTP_PROXY
      # Step 5: Configure Docker to use the proxy
      echo "Configuring Docker to use proxy..."
      sudo sh -c "echo '[Service]\nEnvironment=\"HTTP_PROXY=$HTTP_PROXY\"\nEnvironment=\"HTTPS_PROXY=$HTTPS_PROXY\"\nEnvironment=\"NO_PROXY=localhost,127.0.0.1\"' > /etc/systemd/system/docker.service.d/http-proxy.conf"

      # Step 6: Reload systemd and restart Docker
      echo "Reloading systemd and restarting Docker..."
      sudo systemctl daemon-reload
      sudo systemctl restart docker

      sudo docker pull $1

      # Step 9: Clear Docker proxy configuration
      echo "Clearing Docker proxy configuration..."
      sudo rm /etc/systemd/system/docker.service.d/http-proxy.conf
        systemctl stop v2ray
      sudo systemctl daemon-reload
      sudo systemctl restart docker


