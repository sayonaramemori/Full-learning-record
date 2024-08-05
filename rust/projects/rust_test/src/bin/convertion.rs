fn main() {
    if let Ok(val) = "true".parse::<bool>() {
        println!("Parse Ok val is {val}");
    }else {
        println!("Parse val failed");
    }

}
