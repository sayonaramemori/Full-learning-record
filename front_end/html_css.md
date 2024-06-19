### Block elements  
1. Occupy single line.  
2. Can hold block eles, but inline block cannot hold block eles.  
3. Headline elements can not hold itself.  
4. P eles can not hold block eles.  
```js
<h1> //headlines element
<p>
<div> //Most used container
```


### Text elements  
```
<em>
<strong>
<span>
```

### Image element  
> Modify width or height then the other set with the same proportion.  
```
<img src='' width='' alt=''>
```


### \<a\> raise application  
```
<a href="tel:10086">Telephone</a>
<a href="mailto:jkk@qq.com">Mail</a>
<a href="sms:10086">sms to</a>
```

### Global attribute  
```
<img id='' class='' style='color: green;' title='' dir='' lang=''>
//title is a hint when hold on this elements
//dir default 'ltr', diplay from l to r, you can set it 'rtl'
```

### meta  
```
<meta charset="UTF-8">
//for IE
<meta http-equiv="X-UA-Compatible" content="IE=edge">
//for mobile
<meta name="viewport" content="with=device-width, initial-scale=1.0">
```

### CSS priority  
1. `inline > inner = outter`  
2. The Latter wins when the same priority encountering.  

### CSS Selector  
#### CSS Basic Selector  

##### Wildcard  
```css
* {
    color: yellow;
    font-size: 2px;
}
```
##### Element selector  
```css
h1 {
    color: yellow;
    font-size: 2px;
}
```

##### Class selector  
```css
<!--Multi-class-is-separated-by-space-->
<h1 class='java python'>Hello</h>

.java {
    color: yellow;
    font-size: 2px;
}
.python {
    color: red;
    font-size: 3px;
}
```

##### ID selector  
```css
//ID should not start with number
#line {
    color: red;
    font-size: 3px;
}
```

#### CSS Compound Selector  

##### Intersection Selector  
```css
//&&
//Simply concat them, element selector should be presented first if exists.
p.java {
    color: red;
    font-size: 3px;
}
```
##### Union Selector  
```css
//||
//use comma to separate them
p,
.java {
    color: red;
    font-size: 3px;
}
```

##### Offspring Selector  
```css
//use space to separate
//p is the offspring of elements own class '.java'
.java p{
    color: red;
    font-size: 3px;
}
```

##### Son Selector  
```css 
//use >
.java > p{
    color: red;
    font-size: 3px;
}
```

##### Adjacent Brother Selector  
```css
//use +
//look down the one next .java if is not p, then it not works
.java+p{
    color: red;
    font-size: 3px;
}
```

##### General Brother Selector  
```css
//use ~
//look down and select all p below .java
.java~p{
    color: red;
    font-size: 3px;
}
```

#### CSS Attribute Selector  
```css
//select elements possessing attribute title
[title] {
}
//with specified value
[title="explain"] {
}
//with value starting with e
[title^="e"] {
}
//with value ending with e
[title$="e"] {
}
//contains e
[title*="e"] {
}
```

#### CSS Pseudo-Class Selector  

##### Dynamic Pseudo  
```rust
//for a href, your css code should in sequence with l v h a
//unvisited
a:link {
}
a:visited {
}
a:hover {
}
//click down
a:active {
}
//only for form
input:focus {
}
```

##### Not pseudo-selector  
```css
div.java:not(.python) {
}
```

### Length dimension  
```css
#d2 {
    font-size: 20px;
    width: 1em; //Base on font-size, 20  x   1 = 20px
}

//Relative to its father 
#d3 {
    width: 100%;
    height: 200%;
    font-size: 20%;
}
```

### CSS Display Pattern  
```css  
//inline eles include most of text elements
<a> and <label>
//inline block:
<img> <input> <button> <iframe> <textarea>

span {
    //inline, block or none
    display: inline-block;
}
```
1. block  
    - Occupy single line  
    - Default width: the same with its father
    - Default height: set by content  
    - Size(width,height) can be set by css
2. inline  
    - Not occupy single line  
    - Default width: set by content  
    - Default height: set by content  
    - Size can not be set by css  
3. inline-block  
    - Not occupy single line  
    - Default width: set by content  
    - Default height: set by content  
    - Size can be set by css  
    
### Box Model  
> margin -- border -- padding -- content(width,height)  

> Box size = border + padding + content  

> margin influence the position of the box  


#### Content  
> Its size can be decreased by margin and border without setting size. 
```css
.java {
    //no width or height set
}
```

#### Padding  
> Padding-top and padding-bottom have bad display for inline elements
```css
//setting style like margin  
.java {
    //t b l r
    padding: 20px;
    //tb and lr
    padding: 20px 20px;
    //t, lr and b
    padding: 20px 20px 20px;
    //t r b l -- clockwise  
    padding: 0px 0px 0px 0px;
}
```

#### Border  
```css
.java {
    border-left: 20px red solid;
    border: 20px red solid;
    ...
}
```

#### Margin  

1. Cannot set top and bottom for inline elements.  
2. Can be set negative value, simply cover the former. 
3. Collapse problem: the margin set for sons but gained by their father.  
    - Set marginTop for the first son or set marginBottom for the last son.
    - Solution One: set border or padding for father.(not good)  
    - Solution Two: set overflow: hidden for father.(most used)  
4. Margin is a public place, when a ele set marginBottom and the next set marginTop.(only top and bottom)  


```css
.java {
    //horizontal centering for block elements
    margin: 0px auto;
}
.js {
    //relative to father's width
    margin: 0px 2.5%;
}
```

### Hide elements  
```css
p {
    display: none;
    visibility: show; //or hidden;
}
```

### Style Inheritage  
> Style influencing the layout won't be inherited.  

> For less code.  

```css
body {
    font-size:
    font-weight:
    color:
}
```

### Default Style  
> Priority to style inherited  

> body has default margin  

### Tips for layout  
1. inline and inline-block elements can be treated as text  
    - Thus you can use text-align, line-height and text-indent.  
2. Let son be horizontally centering.  
    - If son is block, set margin:0 auto;
    - If son is inline(inline-block), set text-align:center for father.  
3. Let son be vertically centering.  
    - If son is block, you should calculate the margin top and set overflow:hidden.  
    - If son is inline(inline-block), set height equal to line-height in father stylesheet, then set font-size:0 for father, last set vertical-align:middle for sons.  

### Interval between inline(-block) elements  
> Newline between elements will be parsed as an whitespcae char  

> Solution: set `font-size:0` for father.

### Problem of appariation blankspace of inline-block  
> Arise from the align of inline-block elements with text baseline.  

> Solution: set `vertical` for it.  



### Float  
1. A element floated is out of the document stream.  
2. Size is defaultly expanded by content, can be set. 
3. Not be treated like text.  


### Relative position  
> For slight adjustment.  
```
//Not out of document stream
//relative to the original place
div {
    position: relative;
    left: 200px;
}
```

### Absolute position  
```
//out of document stream  
//relative to the first father set postion: relative;
div {
    position: absolute;
    top: 20px;
}
```

### Fixed position  
```
//relative to the viewer  
div {
    position: fixed;
    top: 0;
}
```

### Sticky position
```
//relative the first father owns scroller  
div {
    position: sticky;
    top: 1px;
}
```

### Special application of position  
```
//width fully fill the father 
//size is not preset
div {
    position: absolute;
    left: 0;
    right: 0;
}

//absolutely centering
//size is preset
div {
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
    margin: auto;
}
```

### Reset the Default style  
```css
* {
    margin: 0;
    padding: 0;
    border: 0 solid transparent;
}
//introduce normalize.css
<link ref='stylesheet' href='path'>
```

### CSS3  

#### New length metric  
```css
.box {
    width: 20vw;
    height: 20vh;
}
```

#### Web Font  

#### 2D Transform  
```css
//move
//Not work for inline elements
//Used for vertical align with position: top 50%
div {
    //relative to its own size
    transform: translateX(10%);
    transform: translateY(10%);
    //or (x,y)
    transform: translate(10%,10%);
}

//scale
div {
    transform: scaleX(1.5); //or scaleY(0~1);
    transform: scale(0.5);  //for x & y
}

//rotate  
div {
    //clockwise direction  
    //axis also rotated 
    transform: rotateZ(20deg); //rotate 2D
}

//transform origin point, influence rotate and scale
div {
    //with key word
    transform-origin: right bottom;
    //with specific value, px is also ok,
    transform-origin: %50 %50;
}
```


#### Transition  
> Include color, length, opacity, shadow and transform.  

```css
div {
    //specify the attributes needing transition
    transition-property: width,height;
    transition-duration: 1s,2s;
}
div {
    //specify all the attributes can be transitioned
    transition-property: all;
    //or 1000ms
    transition-duration: 1s;
    transition-delay: 0.5s;
    //default ease, others include linear, ease-in, ease-out
    transition-timing-function: cubic-bezier(.12,.86,.98,.04);
}
div:hover {
    height: new-value;
    width: new-value;
}
```

#### Animation  
```css
div {
    animation-name: some-name;
    animation-duration: 3s;
    animation-timing-function: cubic-bezier(.12,.86,.98,.04);
    animation-iteration-count: infinite;
    animation-direction: reverse;
    animation-play-state: paused; //running
}

//define a set of key frames
@keyframes myAnimation {
    //first frame
    //0%
    from {

    }
    //last frame
    //100%
    to {
        transform: translate(900px);
    }
}

//more flexable
@keyframes newAnimation {
    0% {

    }
    33% {

    }
    51% {

    }
    100% {

    }
}
```








