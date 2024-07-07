user=`whoami`
echo "Current operation for user: $user"
read -p 'Please ensure the current user:' user
query=`id $user | grep no`
if [ -z $query ]
then
	echo "Now for $user"
    if [ $user = 'root' ]
    then
        prefix="/$user"
    else
        prefix="/home/$user"
    fi
    cp ./.vimrc ${prefix}/.vimrc

    if [ -e ${prefix}/.vim/autoload ]
    then
        cp ./plug.vim ${prefix}/.vim/autoload/
    else
        mkdir ${prefix}/.vim/autoload -p
        cp ./plug.vim ${prefix}/.vim/autoload/
    fi
else
    echo 'no such user, stop running'
fi

