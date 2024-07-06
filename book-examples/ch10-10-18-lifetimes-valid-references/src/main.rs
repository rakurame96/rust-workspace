fn main() {
    let r;
    
    let x = 5;

    r = &x;
    // reference of x is valid until the program execution
    println!("r: {r}");
}
