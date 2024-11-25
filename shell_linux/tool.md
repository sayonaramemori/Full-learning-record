### Scan ip
```shell
//umask is 255.255.255.0 8*3 = 24
nmap -sn 192.168.1.0/24
//or
nmap -sP IP-Range

//umask is 255.255.240.0 2*8+8-4=20
//-v is verbose   
nmap -v -sn 192.168.xx.0/20
```

### Scan Port

> `nmap -sT IP -p-`  

> `nmap -sS IP -p-`  

### Finding plc
> `nmap -sn 192.168.31.0/24 | grep for | cut -d ' ' -f5,6`

### Trace Route  
> `traceroute IP`

### Rename host
> `hostnamectl set-hostname [name]`

### Amazing relaxing tools
1. cowsay -f dragon 'hello'
2. cmatrix
3. bastet
4. tldr  
5. neofetch  
6. btop  
7. fzf  
