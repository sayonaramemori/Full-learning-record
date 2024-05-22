### Install Rust On Linux  
> `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  
    - `cargo --version`  
    - `rustc --version`  
    - 'rustdoc --version`  
    - `rustup update`  

> Proxy is preferable.  
> Using Docker to deploy v2ray  

### Basic Type  
1. Auto-deduce for variable definition is preset.  
2. Using `0x,0o,0b` to denote hexadecimal,octal and binary. 
3. Using `as` to achieve type conversion.  


|Type|Explanation|Val|  
|:--:|:--:|:--:|  
|i8,i16,i32,i64,u8,u16,u32,u64|Integer|42,-5i8,20_922_789_888u64,b'*'|  
|isize,usize|Size the same the machine word||
|f32,f64|Float number|1.8,3.14f32|  
|bool|Boolean|true,false|  
|char|Unicode char with 32bit width|'\n','*'|  
|(char,u8,i32)|Tuple|('%',0x7f,-1)|  
|()|Empty tuple|()|  
|struct S{field:type}|Named struct|S{field:89}|
|struct T(i32,char);|Tuple-like struct|T{120,'x'}|  
|struct E;|No field|E|  
|enum Attend{OnTime,Late(u32)}|Enumeration|Attend::Late(5),Attend::OnTime|  
|Box<Attend>|Pointer|Box::new(Late(15))|  
|&i32,&mut i32|Reference|&s.field,&mut v|  
|String|UTF-8 string|"roman".to_string()|  
|&str|Reference to String|"roman",&s[0..12]|  
|[f64;4]|Array with fixed length|[b' ';256]|  
|Vec<f64>|Vector with desirable length|vec![1,2,3]|  
|&[u8],&mut [u8]|Slice|&v[1..12]|
|&Any,&mut Read|Trait type|&mut file as &mut Read|  
|fn(&str,usize)->isize|Function pointer|String::from|  
|\|para\|{body}|Closure|\|a,b\| a\*a+b\*b|

###
