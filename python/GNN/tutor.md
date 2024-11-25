### Install conda  
- [Github Page](https://github.com/conda/conda)  
- Select your version from the Distribution [page](https://repo.anaconda.com/archive/).  

### Basics of conda  
```
conda create --name env
conda active env
```

### Prerequisite  
```
pip install numpy pandas matplotlib tqdm networkx -i https://pypi.tuna.tsinghua.edu.cn/simple


# Download Chinese Font for matplotlib 

loc=`pip show matplotlib | grep Location | awk '{print $2}'`

wget https://github.com/StellarCN/scp_zh/raw/refs/heads/master/fonts/SimHei.ttf -O ${loc}/matplotlib/mpl-data/fonts/ttf/SimHei.ttf

rm -rf ~/.cache/matplotlib
```

