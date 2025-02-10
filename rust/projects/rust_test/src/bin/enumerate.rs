enum Test{
    A,
    B(String),
    C{x:u8,y:u8},
    D(String,String),
}

impl Test{
    fn print(&self){
        match self{
            Test::A => println!("A"),
            Test::B(_b) => println!("B"),
            Test::C{x,y} => println!("C"),
            Test::D(s1,s2) => println!("D"),
        }
    }
}

fn main(){
    let a = Test::C{x:0,y:0};
    a.print();
}
