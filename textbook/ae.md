## Basic  

### Preview Panel
> Double click to preview or add new files.  
1. Scroll to scale  
2. Drag preview via middle button and `space`  


### Pre settings  
- Click `编辑` and then open `首选项`  
    1. Allow scripts to access network  
    2. Place the cache in D disk instead of C.  

### Mode for Preview Panel  
1. `V` mode for ***position*** or size operation 
2. `Y` mode for ***anchor point*** operation  
3. `W` mode for rotation operation  


### Composition  

#### Composition Panel  
- For every key-frame, click arrow for moving to key-frame  
- Toggle motion blur for smooth.  

#### Layer Attribute  
- `p` for position attr.  
- `s` for scale attr.  
- `r` for rotation attr.  
- `t` for transparent attr.  
- `u` for displaying key-frame  

#### Key Frame  
- Set a value to a attr for a specific time point.  
- Default linear transition.  

#### Nested Composition  
- Use another composition in a specific composition.  

### Output with Alpha  
- After `C-M`, set Quicktime format and RGB+Alpha for output.  

### Time Line  

#### Shortcuts  
1. scale: `- & +`  
2. layer IO: `Alt + []`


### Scissor-Mask  
> Draw specific region and you can set different modes  
- You *must* select a pic-layer first, otherwise a shape-layer.  

#### Pen  
- Press `Q` for rectangular mask.  
    - For drawing a cycle, press `Ctrl + Shift`  
- Press `G` for pen mask.  
- When drawing a mask, press `space` to move this mask.  

#### Operation on mask
1. Press `v` to enter v-mode
2. Select the pin poin to start operation.  

### 遮罩  
> It is a relationship between two pic-layer.  

#### Alpha mask  

#### Luminance mask  
> Only show in the bright or dark area.  

### Link  
- Son is part of Father.  
- All operation on Father will affect the sons.  
- Null object is good choice for father.  

### 3D and Camera  

#### View in 3D  
- Press `c` then press `Alt`  
    - `Left-Click` to rotate  
    - `middle-click` to drag  
    - `Right-click` to change distance  

#### Using Null Object  
- Camera Links to it  
- Scale to control distance.  

#### Split View Panel  
- One is `director` view and the other is `camera` view.  

#### Light  


## Effect  

### 生成  

#### 网格类  
- 棋盘: Black and White  
- 网格: Crossed lines  
- 单元格: For masaic  

#### CC Light Sweep  
- 扫光的文字应为灰度  

#### 勾画  
- For light flow and text wrapper.  

#### 音频频谱  
- Three Type  
- With polar coordinates.  

#### 无线电波  
#### 高级闪电  

### 风格化  

#### CC Repetile  
- Repeat the pic-layer.  

#### 动态拼贴  
- Repeat the pic-layer within the pic.  

#### 马赛克  
#### 查找边缘  
#### 发光  
- 霓虹灯效果: AB  

### 透视  
#### 投影  
#### 斜面 Alpha  
#### CC Sphere & CC Cylinder  
- Convert a plane to a sphere.  


### 模糊
#### 高斯模糊  
#### 定向模糊  
#### CC Radial Fast Blur  
- For light behind the Texts  

### 扭曲  
#### 边角定位  
#### 镜像  
#### 光学补偿  
#### 极坐标  
#### CC Blobbylize  

### 模拟  
#### CC Rainfall  
#### CC Snowfall  

#### 焦散  
- For view under the water  

#### 碎片  

### 杂色和颗粒  

#### 分形噪波
- For light reflected by water surface.  


### 过渡  
#### 线性擦除  
#### 径向擦除
#### 百叶窗  

## Plugins  

### Particular  

### AutoSway  

### Element 3D  

## Expression  
- Basic Javascript Expression  
```js 
index // start from 1
time  // sec metric 
valueAtTime(TimeDelta(second))
loopOut
[x,y] // For position, multiple values are returned within an array  
Math.sin()
```

## Shortcuts  
- Press `Shift` when modifying attr value for large span.  

|KEY|Note|
|:--|:--|
|C-M|Output|  
|b & n|IO for output(length)|  
|m|Toggle Mask Attribute|  
|C-y|New pure-color layer|  
|C-D|Copy and paste|  
|c|Camera, alt with mouse|  
|u|Toggle key-frame display|  
|F9|Easy transition on key-frame curve|  
|`Ctrl+Shift+Alt+G` & `Ctrl+Shift+Alt+H`| Resize pic-layer|  
|`Ctrl+Alt+O`| Set Motion Direction|  

## Experience  

### Path Transform  
- Create two shape-layer and Transform their curve type into Bezier curve. Then copy one's path to the other one's.  
