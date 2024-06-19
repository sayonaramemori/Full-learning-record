### Create a vue project  
```shell
npm create vue@latest
```

### v-html  
```html
<h1 v-html="variable"></h1>
```

### v-if & v-show  
> v-if constructs or deconstructs the element.  
> v-show simply switch the display style.  

### v-on: | @  
```html
<button @click="func">
<button v-on:click="func">
```

### v-bind  
> Usage: v-bind:attr="expr"  
```html
<img v-bind:src="imgurl">
//v-bind can be omitted
<div :class="['blue','big']"></div>
<div :class="{blue: true, big: false}"></div>
<div :style="{width: '400px'}"></div>
```

### v-for  
```html
<ul>
    <li v-for="(item,index) in list" :key="item.id">{{ item }}</li>
    <li v-for="item in list">{{ item }}</li>
<ul>
let list=[]
```

### v-model  
> For form elements, data stream binding.  
```html
<input type="text" v-model="variable">
```

### Instruction suffix  
```
@keyup.enter   //listen to enter up
v-model.trim   //trim blank
v-model.number //to number

```

### setup
```js
//If name is not exported, then name is the file name
<script lang="ts">
    export default {name:"Name"}
<script>
<script setup lang="ts">
    //auto expose
</script>

//omit name exposing, plugin is needed
<script setup lang="ts" name="Person">
npm install vite-plugin-vue-setup-extend -D
vim vite.config.ts
import vueSetup from 'vite-plugin-vue-setup-extend'
plugins:[vueSetup()]
```

### ref for basic type  
```js
import {ref} from 'vue'
let a = ref(1)
```

### reactive for obj type  
```js
import {reactive} from 'vue'
//.value is not need
let car=reactive({brand:'bmw',price:100})
//assign a new obj you should use Object.assign(car,new_car);
```

### computed
```js
//a is also a ref 
let a = computed({
get(){
    //some ref statements
},
set(val){}
});
//call set(val)
a.value = value
```

### watch
```js
import {watch} from 'vue'
//1.watch basic type
//sum is the watched, without sum.value
//oldvalue can be omitted
const stop = watch(sum,(newValue,oldValue)=>{
    if(condition)stop();
})
//2.watch obj type, if only modify field, then new === old
//originally watch the location for an obj.
watch(()=>person,(new,old){},{deep:true})

//3.watch specific field with basic type
watch(()=>person.name,(new,old){});

//4.watch obj in obj
watch(()=>person.car,(new,old){},{deep:true})

//5.multiple watch
watch([()=>person.car,()=>person.name],(new,old){},{deep:true});
```

### defineProps  
```js
//data is also ok
<Person :val='val1' :fun='func'/>
defineProps(['val','fun'])
//call the function
fun('hhh')
//use data
val
```

### defineEmits
> Custom event  
```js
<Person @val1='func1' />
let emit = defineProps(['val1','val2'])
//triger the event and pass parameters
emit('val1',..args)
```

### mitt 
> Mount easily  
```
npm install mitt
const emitter = mitt()
emitter.on('func',()=>{})
onUnmounted(()=>{emitter.off('func')})
```

### ref, $refs and $parent 
```js
<Child ref='child'/>
let child = ref()
child.value.toy= 'java'

//in Child
let toy = ref('java')
let book = ref('java')
defineExpose({toy,book})
```

### provide and inject  
```js
//in father
let cash = ref(100);
provide('money',cash);

//in offspring  
let mon = inject('money',default_value);
```

### anonymous slot  
```js
<Category>
    //put here
</Category>

//in Category
<slot>
  default content
</slot>
```

### Slot with name  
```js
<Category>
    <template v-slot:s1>
        ...
    </template>
    <template v-slot:s2>
        ...
    </template>
</Category>

//in Category
<slot name="s1">
  default content
</slot>
<slot name="s2">
  default content
</slot>
```
### tag ref
```js
import {ref} from 'vue'
//id can be used in template for ref
//<input ref="id">
let id = ref();

//An element of dom
id.value
```

### life stage
```js
import {onBeforeCreate,onCreated,onBeforeMount,onMounted,onBeforeUpdate,onUpdated}
onBeforeMount(()=>{});
```

### Teleport  
















