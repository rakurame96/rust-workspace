use std::mem::size_of_val;

fn main() {
    let b = Box::new(5);
    println!("b = {b}");
    println!("address of b = {:p}", b);
    println!("length of b = {}", size_of_val(&*b));
}
