fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    // dangling reference as x goes out of scope once the execution of inner scope is done. 
    println!("r: {}", r);
}
