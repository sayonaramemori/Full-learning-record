#!/bin/bash
# Set proxy variables
HTTP_PROXY="socks5://127.0.0.1:10808"
HTTPS_PROXY=$HTTP_PROXY

# Step 1: Configure apt to use the proxy
echo "Configuring apt to use proxy..."
sudo sh -c "echo 'Acquire::http::Proxy \"$HTTP_PROXY\";' > /etc/apt/apt.conf.d/01proxy"
sudo sh -c "echo 'Acquire::https::Proxy \"$HTTPS_PROXY\";' >> /etc/apt/apt.conf.d/01proxy"
# Step 2: Add Docker's official GPG key
sudo apt-get update

# Step 8: Clear apt proxy configuration

