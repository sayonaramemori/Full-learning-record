## Essentials  

### Create an Application  
```ts
import { createApp } from 'vue'
// import the root component App from a single-file component.
import App from './App.vue'
const app = createApp(App)
app.mount('#app')

// Multiple application instance
const app1 = createApp({
  /* ... */
})
app1.mount('#container-1')

const app2 = createApp({
  /* ... */
})
app2.mount('#container-2')
```

### Template Syntax  

#### Text interpolation  
```ts
<span>Message: {{ msg }}</span>
```

#### Attribute Binding  
- Use `v-bind` directive  
- If the bound value is `null` or `undefined`, then the attribute will be removed from the rendered element.
```ts  
<div v-bind:id="dynamicId"></div>
// shorthand 
<div :id="dynamicId"></div>

// Binding multiple attributes  
<div v-bind="objectOfAttrs"></div>
const objectOfAttrs = {
  id: 'container',
  class: 'wrapper',
  style: 'background-color:green'
}
```

#### With JS Expressions  
- An expression is a piece of code that can be evaluated to a value  
- In Vue, JS expressions can be used in:  
    1. Inside text interpolations (mustaches)  
    2. In the attribute value of any Vue directives (special attributes that start with v-)
```ts  
{{ number + 1 }}

{{ ok ? 'YES' : 'NO' }}

{{ message.split('').reverse().join('') }}

<div :id="`list-${id}`"></div>

// It is possible to call a component-exposed method inside a binding expression:
<time :title="toTitleDate(date)" :datetime="date">
  {{ formatDate(date) }}
</time>
// Functions called inside binding expressions will be called every time the component updates, so they should not have any side effects, such as changing data or triggering asynchronous operations.
```


#### Directives  
- A directive's job is to reactively apply updates to the DOM when the value of its expression changes.  

##### Arguments  
> Some directives can take an 'arguments', denoted by colon after a directive name. For example, the `v-bind` directive is used to reactively update an HTML attribute:
```ts  
<a v-bind:href="url"> ... </a>

<!-- shorthand -->
<a :href="url"> ... </a>
```

##### Modifiers  
> Modifiers are special postfixes denoted by a dot, which indicate that a directive should be bound in some special way.  
```
@keyup.enter   //listen to enter up
v-model.trim   //trim blank
v-model.number //to number
```

![vue](./img/vuedirective.png)


### Reactivity Fundamentals  

#### Declaring reactive State  
```ts  
import { ref } from 'vue'
const count = ref(0)
const count = ref(0)

console.log(count) // { value: 0 }
console.log(count.value) // 0

count.value++
console.log(count.value) // 1
```


#### \<script setup\>  
> Use it in Single-file Components(SFCs)  

> Top-level imports, variables and functions declared in \<script setup\> are automatically usable in the template of the same component.
```js
<script setup>
import { ref } from 'vue'

const count = ref(0)

function increment() {
  count.value++
}
</script>

<template>
  <button @click="increment">
    {{ count }}
  </button>
</template>
```

#### Why Refs?  
> Vue performs the tracking in its getter, and performs triggering in its setter.  

> Another nice trait of refs is that unlike plain variables, you can pass refs into functions while retaining access to the latest value and the reactivity connection.
```ts  
// pseudo code, not actual implementation
const myRef = {
  _value: 0,
  get value() {
    track()
    return this._value
  },
  set value(newValue) {
    this._value = newValue
    trigger()
  }
}
```

#### Deep Reactivity  
> Refs can hold any value type, including deeply nested objects, arrays, or JavaScript built-in data structures like Map.  
```ts  
import { ref } from 'vue'

const obj = ref({
  nested: { count: 0 },
  arr: ['foo', 'bar']
})

function mutateDeeply() {
  // these will work as expected.
  obj.value.nested.count++
  obj.value.arr.push('baz')
}
```

#### reactive()  
> Unlike a ref which wraps the inner value in a special object, reactive() makes an object itself reactive:

> reactive() converts the object deeply.  

> It is important to note that the returned value from reactive() is a Proxy of the original object, which is not equal to the original object  
```ts  
import { reactive } from 'vue'
const state = reactive({ count: 0 })

<button @click="state.count++">
  {{ state.count }}
</button>;a
```

#### Limitation of reactive  
1. It only works for object types(objects, arrays, and collection types)  
2. Cannot replace entire object:  
```ts  
let state = reactive({ count: 0 })
// the above reference ({ count: 0 }) is no longer being tracked
// (reactivity connection is lost!)
state = reactive({ count: 1 })
```
3. Not destructure-friendly.  
> Due to these limitations, we recommend using ref() as the primary API for declaring reactive state.

### Computed Properties  
1. The returned value is a computed ref  
2. A computed property automatically tracks its reactive dependencies.
```js
//a is also a ref 
let a = computed({
get(_previous){
    //some ref statements
},
set(new){}
});
//call set(val)
a.value = value

//This also means the following computed property will never update, because Date.now() is not a reactive dependency:
const now = computed(() => Date.now())
```

#### Computed Caching vs. Methods  
> The difference is that computed properties are cached based on their reactive dependencies.  

> In comparison, a method invocation will always run the function whenever a re-render happens.


### Class and Style Bindings  
> Vue provides special enhancements when v-bind is used with class and style  

#### Bindings to Objects  
> The syntax means the presence of the active class will be determined by the truthiness of the data property isActive.
```ts
<div :class="{ active: isActive }"></div>

<div
  class="static"
  :class="{ active: isActive, 'text-danger': hasError }"
></div>
const isActive = ref(true)
const hasError = ref(false)

// It will render:
<div class="static active"></div>
```
The bound object doesn't have to be inline:
```ts  
const classObject = reactive({
  active: true,
  'text-danger': false
})
<div :class="classObject"></div>

//This will render:
<div class="active"></div>
```
We can also bind to a computed property that returns an object. This is a common and powerful pattern:
```ts
const isActive = ref(true)
const error = ref(null)

const classObject = computed(() => ({
  active: isActive.value && !error.value,
  'text-danger': error.value && error.value.type === 'fatal'
}))
```

#### Binding Inline Styles  
> `:style` supports binding to JavaScript object values  
```ts  
const activeColor = ref('red')
const fontSize = ref(30)
<div :style="{ color: activeColor, fontSize: fontSize + 'px' }"></div>
```
Although camelCase keys are recommended, :style also supports kebab-cased CSS property keys
```ts  
<div :style="{ 'font-size': fontSize + 'px' }"></div>
```
It is often a good idea to bind to a style object directly so that the template is cleaner:
```ts  
const styleObject = reactive({
  color: 'red',
  fontSize: '30px'
})
<div :style="styleObject"></div>
```
Again, object style binding is often used in conjunction ***with computed properties*** that return objects.

### Conditional Rendering  

#### v-if and v-else  
> A v-else element must immediately follow a v-if or a v-else-if element - otherwise it will not be recognized.  
```ts  
<h1 v-if="awesome">Vue is awesome!</h1>

<button @click="awesome = !awesome">Toggle</button>

<h1 v-if="awesome">Vue is awesome!</h1>
<h1 v-else>Oh no 😢</h1>


<div v-if="type === 'A'">
  A
</div>
<div v-else-if="type === 'B'">
  B
</div>
<div v-else-if="type === 'C'">
  C
</div>
<div v-else>
  Not A/B/C
</div>
```

#### v-if on \<template\>  
```ts  
<template v-if="ok">
  <h1>Title</h1>
  <p>Paragraph 1</p>
  <p>Paragraph 2</p>
</template>
```

#### v-show  
> Only toggles the display CSS property of the element.  
```ts
<h1 v-show="ok">Hello!</h1>
```

### List Rendering  
```ts  
const items = ref([{ message: 'Foo' }, { message: 'Bar' }])
<li v-for="item in items">
  {{ item.message }}
</li>
```
Inside the v-for scope, template expressions have access to all parent scope properties.
```ts  
const parentMessage = ref('Parent')
const items = ref([{ message: 'Foo' }, { message: 'Bar' }])

// supports an optional second alias for the index of the current item:
<li v-for="(item, index) in items">
  {{ parentMessage }} - {{ index }} - {{ item.message }}
</li>
```
You can use destructuring on the v-for item alias similar to destructuring function arguments:
```ts  
<li v-for="{ message } in items">
  {{ message }}
</li>

<!-- with index alias -->
<li v-for="({ message }, index) in items">
  {{ message }} {{ index }}
</li>
```

#### v-for with an object  
```ts  
const myObject = reactive({
  title: 'How to do lists in Vue',
  author: 'Jane Doe',
  publishedAt: '2016-04-10'
})
<ul>
  <li v-for="value in myObject">
    {{ value }}
  </li>
</ul>

<li v-for="(value, key) in myObject">
  {{ key }}: {{ value }}
</li>

<li v-for="(value, key, index) in myObject">
  {{ index }}. {{ key }}: {{ value }}
</li>
```

#### v-for with a Range  
```html  
<span v-for="n in 10">{{ n }}</span>
```

#### v-for with v-if  
```ts
<template v-for="todo in todos">
  <li v-if="!todo.isComplete">
    {{ todo.name }}
  </li>
</template>
```
#### Maintaining state with key  
```html
<div v-for="item in items" :key="item.id">
  <!-- content -->
</div>
```

#### v-for with a Component  
>  In order to pass the iterated data into the component, we should also use props:  
```ts  
<MyComponent
  v-for="(item, index) in items"
  :item="item"
  :index="index"
  :key="item.id"
/>
```

### Event Handling  
- The handler value can be one of the following:
    1. Inline handlers: Inline JavaScript to be executed when the event is triggered (similar to the native onclick attribute).
    2. Method handlers: A property name or path that points to a method defined on the component.

#### Inline Handlers  
```ts  
const count = ref(0)

<button @click="count++">Add 1</button>
<p>Count is: {{ count }}</p>
```

#### Method Handlers  
```ts  
const name = ref('Vue.js')

function greet(event) {
  alert(`Hello ${name.value}!`)
  // `event` is the native DOM event
  if (event) {
    alert(event.target.tagName)
  }
}

<!-- `greet` is the name of the method defined above -->
<button @click="greet">Greet</button>
```

#### Methods in Inline Handlers
```ts  
function say(message) {
  alert(message)
}
<button @click="say('hello')">Say hello</button>
<button @click="say('bye')">Say bye</button>

// Accessing Event Argument in Inline Handlers
<!-- using $event special variable -->
<button @click="warn('Form cannot be submitted yet.', $event)">
  Submit
</button>

<!-- using inline arrow function -->
<button @click="(event) => warn('Form cannot be submitted yet.', event)">
  Submit
</button>

function warn(message, event) {
  // now we have access to the native event
  if (event) {
    event.preventDefault()
  }
  alert(message)
}
```

#### Event Modifiers  

#### Key Modifiers  
> Dot `enter` `tab` `delete` `esc` `space` `up` `down` `left` and `right`  
> System Modifiers Keys: `.ctrl` `.alt` `.shift` and `.meta`(opt or win)
```ts  
<!-- only call `submit` when the `key` is `Enter` -->
<input @keyup.enter="submit" />
<input @keyup.page-down="onPageDown" />
```

### Form Input Bindings  
```ts  
<input
  :value="text"
  @input="event => text = event.target.value">

//The v-model directive helps us simplify the above to:
<input v-model="text">
```

### Watchers  
> Compared to Computed Properties, Watchers Perform "side effects" in reaction to state changes  

#### Watch Source Types  
> `watch`'s first argument can be different types of reactive "sources": it can be a ref(including computed refs), a reactive object, a getter function, or an array of multiple sources:
```ts  
const x = ref(0)
const y = ref(0)

// single ref
watch(x, (newX) => {
  console.log(`x is ${newX}`)
})

// getter
watch(
  () => x.value + y.value,
  (sum) => {
    console.log(`sum of x + y is: ${sum}`)
  }
)

// array of multiple sources
watch([x, () => y.value], ([newX, newY]) => {
  console.log(`x is ${newX} and y is ${newY}`)
})
```
Do note that you can't watch a property of a reactive object like this:
```ts
const obj = reactive({ count: 0 })

// this won't work because we are passing a number to watch()
watch(obj.count, (count) => {
  console.log(`Count is: ${count}`)
})

// instead, use a getter:
watch(
  () => obj.count,
  (count) => {
    console.log(`Count is: ${count}`)
  }
)
```

#### Deep Watchers  
- When you call watch() directly on a reactive object, it will implicitly create a deep watcher - the callback will be triggered on all nested mutations:
```ts  
const obj = reactive({ count: 0 })

watch(obj, (newValue, oldValue) => {
  // fires on nested property mutations
  // Note: `newValue` will be equal to `oldValue` here
  // because they both point to the same object!
})

obj.count++
```

#### Eager Watchers  
- `watch` is lazy by default: the callback won't be called until the watched source has changed
- We can force a watcher's callback to be executed immediately by:  
```ts  
watch(
  source,
  (newValue, oldValue) => {
    // executed immediately, then again when `source` changes
  },
  { immediate: true }
)
```

#### Once Watchers  
```ts  
watch(
  source,
  (newValue, oldValue) => {
    // when `source` changes, triggers only once
  },
  { once: true }
)
```


#### watchEffect()  
- `watchEffect()` allows us to track the callback's reactive dependencies automatically, like computed properties.  
- The option immediate: true is specified implicitly.  
```ts  
// Here, the callback will run immediately, there's no need to specify immediate: true
watchEffect(async () => {
  const response = await fetch(
    `https://jsonplaceholder.typicode.com/todos/${todoId.value}`
  )
  data.value = await response.json()
})
```

#### watch vs. watchEffect  
- `watch` only tracks the explicitly watched source. It won't track anything accessed inside the callback. In addition, the callback only triggers when the source has actually changed. `watch` separates dependency tracking from the side effect, giving us more precise control over when the callback should fire.  
- `watchEffect`, on the other hand, combines dependency tracking and side effect into one phase. It automatically tracks every reactive property accessed during its synchronous execution. This is more convenient and typically results in terser code, but makes its reactive dependencies less explicit.  


### Template Refs  

### Components Basic  

#### Passing Props  
```ts  
<!-- BlogPost.vue -->
<script setup>
defineProps(['title'])
</script>

<template>
  <h4>{{ title }}</h4>
</template>

// Get the return value
const props = defineProps(['title'])
console.log(props.title)

// In parent component  
const posts = ref([
  { id: 1, title: 'My journey with Vue' },
  { id: 2, title: 'Blogging with Vue' },
  { id: 3, title: 'Why Vue is so fun' }
])
<BlogPost
  v-for="post in posts"
  :key="post.id"
  :title="post.title"
 />
```

#### Listening to Events  
```ts  
<BlogPost
  ...
  @enlarge-text="postFontSize += 0.1"
 />

<!-- BlogPost.vue -->
<script setup>
const emit = defineEmits(['enlarge-text'])
emit('enlarge-text')
</script>
```

### Lifecycle Hooks  
```js
import {onBeforeCreate,onCreated,onBeforeMount,onMounted,onBeforeUpdate,onUpdated}
onBeforeMount(()=>{});
```

## Components In-Depth  

### Registration  

### Props  
```ts  
// in <script setup>
defineProps({
  // Basic type check
  //  (`null` and `undefined` values will allow any type)
  propA: Number,
  // Multiple possible types
  propB: [String, Number],
  // Required string
  propC: {
    type: String,
    required: true
  },
  // Required but nullable string
  propD: {
    type: [String, null],
    required: true
  },
  // Number with a default value
  propE: {
    type: Number,
    default: 100
  },
  // Object with a default value
  propF: {
    type: Object,
    // Object or array defaults must be returned from
    // a factory function. The function receives the raw
    // props received by the component as the argument.
    default(rawProps) {
      return { message: 'hello' }
    }
  },
  // Custom validator function
  // full props passed as 2nd argument in 3.4+
  propG: {
    validator(value, props) {
      // The value must match one of these strings
      return ['success', 'warning', 'danger'].includes(value)
    }
  },
  // Function with a default value
  propH: {
    type: Function,
    // Unlike object or array default, this is not a factory
    // function - this is a function to serve as a default value
    default() {
      return 'Default function'
    }
  }
})
```

### Component v-model  
```ts  
<!-- Child.vue -->
<script setup>
const model = defineModel()

function update() {
  model.value++
}
</script>

<template>
  <div>Parent bound v-model is: {{ model }}</div>
  <button @click="update">Increment</button>
</template>

<!-- Parent.vue -->
<Child v-model="countModel" />
```

#### v-model arguments  
```ts  
<MyComponent v-model:title="bookTitle" />

<!-- MyComponent.vue -->
<script setup>
const title = defineModel('title')
</script>

<template>
  <input type="text" v-model="title" />
</template>
```

### Fallthrough Attribute  

### Slots  

### Provide and Inject  


















