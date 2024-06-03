use std::thread::spawn;
fn main() {
    let mut con = vec![];
    let s = String::new();
    for i in 0..9 {
        con.push(spawn(move || println!("iter:{}",i)));
    }
    for i in con {
        i.join().unwrap();
    }
}
