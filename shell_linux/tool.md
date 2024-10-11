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

### Set static IP for Ubuntu
```shell
cd /etc/netplan
sudo vim 01-network-manager-all.yaml
'''Write below to the file
'''Gateway is your router ip
'''Specify eth-n, use ifconfig to check which to select.
'''To remove, simply delete the item ethernets.
network:
    version: 2
    renderer: NetworkManager
    ethernets:
        eth3:
            dhcp4: no
            addresses: [192.168.31.66/24]
            gateway4: 192.168.31.1
            nameservers:
                addresses: [223.5.5.5,114.114.114.114]
```

### Amazing relaxing tools
1. cowsay -f dragon 'hello'
2. cmatrix
3. bastet
