#!/bin/bash

# Set proxy variables
HTTP_PROXY="socks5://127.0.0.1:10808/"
HTTPS_PROXY=$HTTP_PROXY

# Step 1: Configure apt to use the proxy
echo "Configuring apt to use proxy..."
sudo sh -c "echo 'Acquire::http::Proxy \"$HTTP_PROXY\";' > /etc/apt/apt.conf.d/01proxy"
sudo sh -c "echo 'Acquire::https::Proxy \"$HTTPS_PROXY\";' >> /etc/apt/apt.conf.d/01proxy"

# Step 2: Add Docker's official GPG key
echo "Adding Docker's official GPG key..."
sudo apt-get update
sudo apt-get install -y ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc --proxy "$HTTP_PROXY"
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Step 3: Add the Docker repository to apt sources
echo "Adding Docker repository to apt sources..."
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo \"$VERSION_CODENAME\") stable" | \
      sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

      # Step 4: Install Docker
      echo "Installing Docker..."
      sudo apt-get update
	  sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

      # Step 5: Configure Docker to use the proxy
      echo "Configuring Docker to use proxy..."
      sudo mkdir -p /etc/systemd/system/docker.service.d
      sudo sh -c "echo '[Service]\nEnvironment=\"HTTP_PROXY=$HTTP_PROXY\"\nEnvironment=\"HTTPS_PROXY=$HTTPS_PROXY\"\nEnvironment=\"NO_PROXY=localhost,127.0.0.1\"' > /etc/systemd/system/docker.service.d/http-proxy.conf"

      # Step 6: Reload systemd and restart Docker
      echo "Reloading systemd and restarting Docker..."
      sudo systemctl daemon-reload
      sudo systemctl restart docker

      # Step 7: Test Docker installation by pulling an image
      echo "Testing Docker installation by pulling the hello-world image..."
      docker pull hello-world

      # Step 8: Clear apt proxy configuration
      echo "Clearing apt proxy configuration..."
      sudo rm /etc/apt/apt.conf.d/01proxy

      # Step 9: Clear Docker proxy configuration
      echo "Clearing Docker proxy configuration..."
      sudo rm /etc/systemd/system/docker.service.d/http-proxy.conf
      sudo systemctl daemon-reload
      sudo systemctl restart docker

      echo "Script completed successfully."

	  sudo docker run --name ciallo hello-world
	  sudo docker stop ciallo
