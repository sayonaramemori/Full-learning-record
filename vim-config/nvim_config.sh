#!/bin/bash
mkdir -p ~/.config/nvim
rm -rf ~/.config/nvim/*
cp ./init.lua ~/.config/nvim/init.lua
cp -r ./lua ~/.config/nvim/


