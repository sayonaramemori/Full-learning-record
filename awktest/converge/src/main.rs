fn main() {
    let mut i = std::env::args().skip(1);
    let zh = i.next().unwrap();
    let en = i.next().unwrap();
    let zh = std::fs::read_to_string(zh).unwrap();
    let en = std::fs::read_to_string(en).unwrap();
    let mut res = "zh,en\n".to_string();
    zh.lines().zip(en.lines()).map(|(z,e)|{
        let temp = format!("{z},{e}\n");
        res.extend(temp.chars());
    }).last();
    println!("{res}");
}
