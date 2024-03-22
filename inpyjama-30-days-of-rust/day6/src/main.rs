// fn main() {
//     let x = 5;      // x is an integer (type inferred)
//     println!("Value of x is {}", x);
// }

static mut COUNTER: u32 = 0; // Mutable static variable

fn increment_counter() {
    // COUNTER += 1; // This line is unsafe!
    unsafe {COUNTER += 1;} // This line is unsafe!
}

fn main() {
    increment_counter();
    unsafe {
        println!("{}", COUNTER); // Will this always print 1?
    }
}