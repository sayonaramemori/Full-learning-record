### Install rust on WSL
> `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  
> Run `rustup update` to update Rust.  
> Run `rustup self uninstall` to uninstall Rust.


### Quick start
> `cargo new project_name`  
> [crates](https://crates.io), you can easily access rust package here  
> Run `cargo build` in your project directory.  
> Run `cargo run`.  
> Run `cargo update` to update dependecy package.

### Variable  
1. Variable declared in rust is default immutable. Add mut keyword to declare a mut variable.  
2. A variable can be redeclared in a code block, the latter shadowing the former.  
3. Basic Data Type built in Rust, including int, float, bool and char. The Integer includes i series and u series. The Float includes f32 and f64, supporting mathmatic calculations.
4. Compound Data Type built in Rust, including tuple and array.  
```rust
fn main(){
    let x: i32=1;
    let y: f64=9;
    let x: usize = 2;               //depend on the platform
    let y: isize = 4;               //depend on the platform
    //-----tuple----
    let z = (43,5.1,0);
    let (a,b,c) = tup;              //deconstruct a tuple
    let d = z.0;                    //access by index
    //-----array----
    let e = [1,2,3,4];              //the same data type is needed
    let f: [i32;5] = [1,2,3,4,5];   //[type;number]
    let g = [3;5];                  //[val;number] 
}
```

### Function  
> Often named with snake case  
> Statement and Expression in Rust
```rust
fn compare_two_num(rhv:i32,lhv:i32) -> bool {
    //This is a expression, with a result returned.
    rhv>lhv
}
fn compare_num(rhv:f32,lhv:f32)->bool{
    //This is a statement, without a result returned, so return value is needed.
    let res = rhv>lhv;
    //res
    return res;
}
```

### Control Flow  
> Integer type will not transform to bool in Rust. Explicit bool is needed.
```rust
fn test(){
    let a = 10;
    if a>11 {
       //do something 
    }else{
       //do something
    }

    //A tricky usage, like ?: in Cpp
    let condition = true;
    let number = if condition {
        5
    }else{
        6
    }

    while condition {
        //do something
    }

    //loop while and for
    loop {
        println!("again!");
    }

    //loop with result
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter==10 {
            break counter * 2;
        }
    };

    let a = [3;5];
    for ele in a.iter() {
        println!("{}",ele);
    }
    for num in (1..4).rev() {
        println!("{}",num);
    }
}
```


### Ownship  
> A variable is valid from its declaration to the end of the code block. The Drop function will be called when that happens.
1. Every value in Rust corresponds to a variable as an ownner.  
2. At a time, a value corresponds to only a variable.  
3. When ownner goes out of his block, his value will be cleared.  

### Copy trait  
> No ownship changed when assignment happens.
1. All basic data type (Exclude array).
2. Tuple consists of basic data type.

### Ownship Exchange
1. Simply assignment  
2. Simply function call, without Ref or Slice. And its return.


### Ref and mut Ref
> No ownship exchanged.  
1. Ref always is valid (The rust compiler guards it).
2. In a given time, you can have only a mut ref or multiple immut refs.(Prevent Data Competition)
```rust
fn cal(s:&String)->usize{
    s.len()
}
fn main(){
    let s1 = String::from("Hello");
    //s1 is still valid when call the function
    let len = cal(&s1);
}
```


### Slice for String and array
```rust
let s = String::from("hello world");
let hello = &s[0..5]; //or [..5]
let world = &s[6..]

//temp can convert to &[String] using &temp[..]
let temp:Vec<String> = Vec::new();
//String literal value is &str, like const string* in cpp
let s = "hello world";
```

### Struct and Tuple Struct  
```rust
//define a struct
struct User{
    name: String,
    email: String,
    active: bool,
}

//declare a struct variable
let mut user1 = User{
    email: String::from("sb"),
    name: String::from("hh"),
    active: true,
};
//user1 is mutable or totally immutable.

//access by field
user1.name = String::from("java");

fn build_user(email:String,username: String)->User{
    User {
        email,
        username,
        active: true,
    }
}

let user2 = User{
    username: String::from("cpp"),
    ..user1
}

//struct tuple
struct Color(i32,i32,i32);
let black = Color(0,0,0);
```

### Method on Struct  
```rust
struct Rectangle{
    width: f64,
    height: f64,
}

impl Rectangle{
    fn area(&self)->f64{
        self.width*self.height
    }
    fn can _hold(&self,other:&Rectangle)->bool{
        self.width>other.width&&self.height>other.height
    }
    //static method, that means no self parameter.
    fn square(size:u32) -> Rectangle{
        Rectangle{width:size,height:size}
    }
}
```

### Enum and Match  
```rust
enum IpAddr{
    V4(String),
    V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

//impl can be also applied to enum
//Different data field, quit field binds nothing.
enum Message {
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

//Option enumeration in std-lib.
// enum Option<T>{
//  Some(T),
//  None,
//}

fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

//match must be exhausitive
//_ for other situations
let some_u8_val = Some(0u8);
match some_u8_val {
    Some(3) => println!("get it"),
    _ => (),
}

let five = Some(5);
let six = plus_one(five);
```

### Package, Crate and Module  
- package --- A Cargo function for building, testing and sharing crates.  
- crate --- A tree shape module to generate binary or executable file.  
- module --- They are used to control scope.  
- path --- Name for struct, function and module.

```rust
//When you run cargo new project, the src/main.rs is treated as a root binary unit-package.
//And src/lib.rs is treated as a root lib unit-package.

//Run cargo new --lib [lib-name] to create a lib
//Use mod to create module in src/lib.rs
//In a module, you can define functions, structures, enumerations, const variables and traits.
mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist() {}
        //This function below is still private
        fn seat_at_table() {}
    }
    //serving is private
    mod serving {
        fn take_order(){}
        fn serve_order(){}
    }
}
// main.rs and lib.rs construct a module named 'crate' respectively.
// For lib.rs the module tree is like:
crate
|_____front_of_house
        |
        |----hosting
        |      |-----add_to_waitlist
        |      |
        |      !_____seat_at_table
        |
        |----serving
               |-----take_order
               |
               !_____serve_order
        
//Specify a path
//1.absolute path -- start from crate
//2.relative path -- use self and super keyword

pub fn eat_at_restaurant() {
    //absolute path -- they are defined in the same module(crate)
    crate::front_of_house::hosting::add_to_waitlist();
    //relative path -- brothers can see each other mutually.
    front_of_house::hosting::add_to_waitlist();
    
}
//Items defined in modules is private default, sons can access his father but father cannot access.
//Use pub to mark items(fn,struct .etc) to expose them outside.


//use super
fn serve_order(){}
mod back_of_house() {
    fn fix_incorrect_order() {
        //supper corresponds to crate here
        supper::serve_order();
    }
}


//for struct in module, pub a struct but its field is still private, you can specify for every field.
//but for enumeration, pub a enum meaning all fields is public. This is a little different from struct.
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        //private here, so you can access it in other function directly.
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}


//Use 'use' keyword to import functions or items. Here is a module name.
//absolute path
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//Use 'as' to avoid name crushion
use std::fmt::Result;
use std::io::Result as IoResult;

//Use 'pub use' to re-export -- Items imported is valid in current scope privately.
pub use crate::front_of_house::hosting;
//Outside code can call hosting::add_to_waitlist now.

//Import in a line
use std::io::{self, Write};
```

### Collections  

#### Vec
```rust
let v: Vec<i32> = Vec::new();
let mut v = vec![1,2,3];
v.push(4);
let a = &v[100];    //lead to panic
let a = v.get(100); //None

//travel vec, with deRef operator *
for i in &mut v {
    *i += 50;
}

enum SpredsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
];
```

#### String  
```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2); //receive a parameter with type str&, no ownship exchanged.
s1.push('*');    //receive a char type.
let s = format!("{}-{}",s1,s2); //no wonship exchanged.
//You can travel a string like:
//or s.bytes()
for c in s.chars() {
    println!("{}",c);
}
```

### Error handler in Rust  
> Result<T,E> and panic!  
```rust
enum Result<T,E> {
    Ok(T),
    Err(E),
}

fn main(){
    let f = File::open("hello.txt");
    //case 1
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("There was problem creating file: {:?}",e),
            },
            other_error => panic!("There was problem operating the file: {:?}",error),
        },
    };


    //case 2
    let f = File::open("hello.txt").unwrap();      //call panic! when Err is returned.
    let f = File::open("hello.txt").expect("MSG"); //call panic! when Err is returned with your msg.
}

//use ? to spread error
fn read_username_from_file() -> Result<String,io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

```


### Generic type, trait and life span  
#### Generic Type
```rust
//A generic function, list is a slice with T type
fn largest<T>(list: &[T]) -> T {
    //do something here
}
//define a generic type struct
struct Point<T,U> {
    x: T,
    y: U,
}
impl<T,U> Point<T,U> {
    //A new instance of Point will be created.
    fn mixup<V,W>(self,other: Point<V,W>) -> Point<T,W> {
        //write here 
    }
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}
//implement for a specific type
impl Point<f32,f32>{
    //write here
}
```

#### Trait  
> Like interface so much.  
```rust
//define a trait  
pub trait Summary{
    fn summarize(&self) -> String;
}
pub struct NewsPaper{
    title: String,
    content: String,
}
impl Summary for NewsPaper{
    fn summarize(&self) -> String{
        //implement here
    }
}
//define a trait with default implement
pub trait Summary{
    fn summarize(&self) -> String{
        String::from("Read more...")
    }
}
//NewsPaper now possessd this trait --- summarize
impl Summary for NewsPaper{}

//add trait to parameters
pub fn notify(item: impl Summary) {
    //write here
}
//more offical
fn some_fn<T,U>(t:T, u:U) -> impl Summary
    where T: Display + Clone,
        U: Clone + Debug
{
    //write here
}

impl<T:Display + PartialOrd> Pair<T> {
    //implemet here
}
//implement trait for generic type
impl<T: Display> ToString for T{

}
```

#### Generic Trait  
```rust

```


#### Life span   
> Rust computes life-span for every ref parameters based on the rules below.
1. Every input ref parameters have its own life-span parameters  
2. When only one input ref parameter exists, its life-span will be assigned to all output parameters.  
3. When `self` parameter exists in several input ref parameters, its life-span ....
```rust
//add lifetime parameters explictly
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{}
//In struct 
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    ...
}
//static life span 
let s: &'static str = "I have a static lifetime.";
```



### Auto Debug  
> Run `cargo test`
```rust
//This module will be contained only when cargo test
//configuration -- including all functions in this module
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2+2,4);
    }
}

//assert!(bool_expr)
//if true, nothing will happen

//a and b must implement PartialEq and Debug trait
assert_eq!(a,b);
assert_ne!(a,b);
```

### std::env  
```rust
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
}
```


### Iterator and Closure  
#### Iterator  
```rust
let my_closure = |num| -> bool {num+1};

//Fn series trait, closure type is auto deduced
//FnOnce FnMut Fn
//use move to specify FnOnce
let x = vec![1,2,3];
//x lose its ownship
let equal_to_x = move |z| z==x;


//High benchmark 
let buffer: &mut [i32];
let coefficients: [i64;12];
let qlp_shift: i16;
for i in 12..buffer.len() {
    let prediction = coefficient.iter()
                        .zip(&buffer[i - 12..i])
                        .map(|(&c,&s)| c*s as i64)
                        .sum<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

#### Implement your Iterator  
```rust
//Iterator trait  
pub trait Iterator {
    type Item;
    fn next (&mut self) -> Option<Self::Item>;
}

//iter,iter_mut,into_iter(get ownship)
//generate other iterator adaptor  
let v1 = vec![1,2,3];
let adaptor = v1.iter().map(|x| x+1);
let v2 = adaptor.collect();

//Generate other adaptor 
//filter map enumerate skip

//Consuming adaptor
//collect sum 

//implement it
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter{ count:0 }
    }
}
impl Iterator for Counter {
    //you iterator return type when consuming.
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        }else{
            None
        }
    }
}
```

### Smart Pointer  
```rust

```


### OOP with Rust  
#### Use trait   
```rust
pub trait Draw {
    fn draw(&self);
}
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}
impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {

    }
}
```


### Match Pattern  
```rust
//if let
fn main() {
    let favarite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "43".parse();
    if let Some(color) = favarite_color {
        println!("ok{}",color);
    }else if is_tuesday{
        println!("today is tuesday");
    }else if let Ok(age) = age {
        println!("Using purple for {}",age);
    }else{
        println!("nothing matched");
    }
}

//while let
let mut stack = vec![1,2,3];
while let Some(top) = stack.pop() {
    println!("{}",top);
}

//match literal value including char
let x = 1;
match x {
1 => {},
2 => {},
_ => {},
}

//multiple pattern
match x {
1|3 => {},
2 => {},
_ => {},
}

//use ... to match a range [] ,only literal num
match x {
1 ... 9 => {}, //    1|2|3...8|9
10 => {},
_ => {},
}

//use ..= to match a range [], only literal num and char
let x = 'c';
match x {
    'a' ..= 'z' => println!("little alpha"),
    'A' ..= 'Z' => println!("Big alpha"),
    _ => println!("Not a alpha"),
}

//undestruct a struct
struct Point{
    x: i32,
    y: i32,
}

let p = Point::new();
let Point {x,y} = p;
//use _ to ignore a val
//use .. to ignore the rest
let Point {x,..} = p;


//add a if guard
let num = Some(4);
match num {
    Some(x) if x<5 => {},
    _ => (),
}
```




