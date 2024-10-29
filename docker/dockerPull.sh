# Set proxy variables
HTTP_PROXY="http://192.168.31.113:10809/"
HTTPS_PROXY=$HTTP_PROXY
      # Step 5: Configure Docker to use the proxy
      echo "Configuring Docker to use proxy..."
      sudo mkdir -p /etc/systemd/system/docker.service.d
      sudo sh -c "echo '[Service]\nEnvironment=\"HTTP_PROXY=$HTTP_PROXY\"\nEnvironment=\"HTTPS_PROXY=$HTTPS_PROXY\"\nEnvironment=\"NO_PROXY=localhost,127.0.0.1\"' > /etc/systemd/system/docker.service.d/http-proxy.conf"


      # Step 6: Reload systemd and restart Docker
      echo "Reloading systemd and restarting Docker..."
      sudo systemctl daemon-reload
      sudo systemctl restart docker

      docker pull $1

