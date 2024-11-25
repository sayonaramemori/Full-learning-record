### Install cuda  
- Recommend to use `install.run` to install cuda-toolkit, with which be cautious with the driver installation.  
- Set PATH and LD_LIBRARY_PATH  

### Clear  
> Run `cuda-uninstaller` for `install.run`  


### Test  
> Go to [sample](https://github.com/nvidia/cuda-samples)  
```shell
nvcc --version
git clone https://github.com/NVIDIA/cuda-samples.git
cd [sample-dir]
make
```
