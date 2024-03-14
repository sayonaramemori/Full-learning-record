user=`whoami`
echo "Current operation for user: $user"
read -p 'Please ensure the current user:' user
echo "Now for $user"
prefix="/home/$user"
cp ./.vimrc ${prefix}/.vimrc

if [ -e ${prefix}/.vim/autoload ]
then
    cp ./plug.vim ${prefix}/.vim/autoload/
else
    mkdir ${prefix}/.vim/autoload -p
    cp ./plug.vim ${prefix}/.vim/autoload/
fi

