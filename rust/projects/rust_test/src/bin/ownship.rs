fn get(s:String){
    println!("{}",s);
}

#[derive(Clone)]
struct body {
    age:u8,
    name:String,
}

impl body {
    fn new()->body{
        body{
            age:0,
            name:"java".to_string(),
        }
    }
}

fn test(){
    let v=vec![1,2,3,4];
    for mut i in v {
        i=i+9;
        print!("{} ",i);
    }
    //v is uninitialized now
    //v is array now, without copy trait
    let v:[body;2]=[body::new(),body::new()];
    for mut i in v {
        println!("{} {}",i.age,i.name);
    }
    //v is invalid
    //println!("{}",v.len());
}

fn main() {
    let x = String::from("java");
    get(x);
    test();
    //x is uninitialized now
    //print!("{}",x);
}
