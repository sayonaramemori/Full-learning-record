### Install zsh  
```shell
# 安装 Zsh
sudo apt install zsh

# 将 Zsh 设置为默认 Shell
# chsh -s /bin/zsh
```

### Install Oh My Zsh  
```shell
wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh
sudo bash install.sh
```

### Config  
```shell
git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k

plugins=(
  git zsh-autosuggestions zsh-syntax-highlighting copypath
)
ZSH_THEME="powerlevel10k/powerlevel10k"
```
