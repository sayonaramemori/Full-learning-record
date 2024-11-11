#!/bin/bash
curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim-linux64.tar.gz &&
sudo rm -rf /opt/nvim &&
sudo tar -C /opt -xzf nvim-linux64.tar.gz &&
cat << 'EOF' >> ~/.bashrc
export PATH="$PATH:/opt/nvim-linux64/bin"
EOF
source ~/.bashrc
mkdir -p ~/.config/nvim
rm -rf ~/.config/nvim/*
cp ./init.lua ~/.config/nvim/init.lua
cp -r ./lua ~/.config/nvim/


