### Basic  
![iptables](./iptables.jpg)
```shell  
iptables [-t filter] -L
iptables -t nat -L

iptables -t filter -I INPUT -s IP -p tcp -dport 3306 -j DROP
iptables -t filter -D INPUT index

#prevent docker
iptables -t filter -I DOCKER -s IP -p tcp -dport [container-port] -j DROP
```

### Architecture  
1. Four tables with five chains  
2. Table order: raw --> mangle --> nat --> filter  

```mermaid
---
    title: Top -> Bottom Traffic road map
---
flowchart TB
    A{Network} -->|IN| B[Table: nat <br> Chain: PREROUTING]
    B -->|TO ME| D[Table: filter <br> Chain: INPUT]
    B -->|NOT TO ME| E[Table: filter <br> Chain: FORWARD]
    D --> G[Local Process]
    G --> H[Table: nat <br> Chain: OUTPUT]
    H --> I[Table: filter <br> Chain: OUTPUT]
    I & E --> K[Routing decision]
    K -->|OUT| L{NetWork}
```
