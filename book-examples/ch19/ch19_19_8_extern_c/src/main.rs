extern "C" {
    // since mingw is mentioned in the path, Rust refers directly to the C Libraries
    fn abs(input: i32) -> i32;

    fn sqrt(input: f64) -> f64;
}

fn main() {
    unsafe {
        println!("
        Absolute value of -3 according to C: {}", 
        abs(-3));
    }

    unsafe {
        println!("
        Square Root of 192 according to C: {}",
        sqrt(192.0));
    }
}
