fn main() {
    let s = Some(String::from("Hello!"));

    //  This code works just fine because we never bind s to anything; it isn’t moved.
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}
