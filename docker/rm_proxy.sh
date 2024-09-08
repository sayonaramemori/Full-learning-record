echo "Clearing apt proxy configuration..."
sudo rm /etc/apt/apt.conf.d/01proxy
sudo systemctl restart docker
sudo apt-get update
