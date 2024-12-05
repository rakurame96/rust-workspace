fn main() {
    let mut num = 5;

    // no unsafe keyword in this code. 
    // We can create raw pointers in safe code; 
    // we just can’t dereference raw pointers outside an unsafe block
    let r1 = &num as *const i32;        // immutable raw pointer
    let r2 = &mut num as *mut i32;        // mutable raw pointer

    println!("r1: {:?}, r2: {:?}", r1, r2);

    // dereferencing raw pointers outside of unsafe block is not allowed. below code will trigger the error
    // this operation is unsafe and requires an unsafe function
    // or block dereference of raw pointer is unsafe and requires unsafe function 
    // or block raw pointers may be null, dangling or unaligned; 
    // they can violate aliasing rules and cause data races: all of these are undefined behavior
    // println!("r1: {:?}, r2: {:?}", *r1, *r2);       // uncomment to see the error
}
