#!/bin/bash
if [ -e ./lf ]; then
    if [ -e /etc/lf ]
    then
        echo 'Skipped: path /etc/lf exists already'
    else
        mkdir /etc/lf -p
        echo 'Done:    create /etc/lf'
    fi

    if [ -e /etc/lf/lfrc ]
    then
        echo 'Skipped: configuration file exists already'
    else
        cp ./lfrc /etc/lf/
        echo 'Done:    copy configure to /etc/lf/lfrc finished'
    fi
    res=`cat /etc/profile | grep lfcd`
    if [ -n "$res" ]
    then
        echo 'Skipped: lfcd existed in /etc/profile already'
    else
        cat ./lfcd >> /etc/profile
        echo 'Done:    append lfcd to /etc/profile finished'
    fi
    source /etc/profile
    if [ -e /usr/bin/lf ]
    then 
        echo 'Skipped: /usr/bin/lf is already exist'
    else
        cp ./lf /usr/bin/
        echo 'Done:    copy lf to /usr/bin finished'
    fi 
    echo "Config OK, use ra to run lfcd, use q to return to the terminal"
else
    echo "Please download lf binary file first and ensure compatible"
fi
