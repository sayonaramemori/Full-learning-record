#!/bin/bash
if [ -e ./lf ]; then
    if [ -e /etc/lf ]
    then
        echo 'path /etc/lf exists already'
    else
        mkdir /etc/lf -p
        echo 'create /etc/lf'
    fi

    if [ -e /etc/lf/lfrc ]
    then
        cat ./lfrc >> /etc/lf/lfrc
        echo 'append configure to lfrc finished, you should chech it manually'
    else
        cp ./lfrc /etc/lf/
        echo 'copy configure to /etc/lf/lfrc finished'
    fi
    res=`cat ~/.bashrc | grep lfcd`
    if [ -n "$res" ]
    then
        echo 'lfcd existed in .bashrc already'
    else
        cat ./lfcd >> ~/.bashrc
        echo 'append lfcd to .bashrc finished'
    fi
    if [ -e /usr/bin/lf ]
    then 
        echo '/usr/bin/lf is already exist'
    else
        cp ./lf /usr/bin/
        echo 'copy lf to /usr/bin finished'
    fi 
    source ~/.bashrc
    echo "Config OK, use ra to run lfcd, use q to return to the terminal"
else
    echo "Please download lf binary file first and ensure compatible"
fi
