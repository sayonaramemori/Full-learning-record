### Clear  
sudo apt-get --purge remove "*cublas*" "cuda*" "nsight*" "nvidia*"
sudo apt-get autoremove
sudo apt-get clean

### Install cuda  
1. Click me to go [cuda-website](https://developer.nvidia.com/cuda-toolkit-archive)
2. Select the version you prefer.(network deb)  
3. Follow his wizards and reboot after finishing.  
4. Click detailed installation options  
5. Set PATH and LD_LIBRARY_PATH  

### Test  
> Go to [sample](https://github.com/nvidia/cuda-samples)  
```shell
nvcc --version
git clone https://github.com/NVIDIA/cuda-samples.git
cd [sample-dir]
make
```
