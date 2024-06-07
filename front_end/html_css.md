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
    
### 










