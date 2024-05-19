use std::env;

fn main() {
    // this method needs to be inside main() method
    env::set_var("RUST_BACKTRACE", "1");    
    
    println!("Panic! macro example");

    let v = vec![1, 2, 3];
    v[99];
}
