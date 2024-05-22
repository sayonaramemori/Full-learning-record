fn main() {
    //u8 not Unicode
    let method :&[u8;3]= b"GET";
    assert_eq!(method,&[b'G',b'E',b'T']);
    println!("ok");
}
