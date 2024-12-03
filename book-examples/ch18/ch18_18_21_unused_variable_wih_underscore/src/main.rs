fn main() {
    let s = Some(String::from("Hello!"));
    
    // We’ll receive an error because the s value will still be moved into _s, which prevents us from using s again
    if let Some(_s) = s {
        println!("found a string");
    }

    // comment below line to see the compile error
    // println!("{:?}", s);
}
