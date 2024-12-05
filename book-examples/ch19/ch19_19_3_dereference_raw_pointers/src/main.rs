fn main() {
    let mut num = 5;

    // no unsafe keyword in this code. 
    // We can create raw pointers in safe code; 
    // we just can’t dereference raw pointers outside an unsafe block
    let r1 = &num as *const i32;        // immutable raw pointer
    let r2 = &mut num as *mut i32;        // mutable raw pointer

    println!("r1: {:?}, r2: {:?}", r1, r2);

    // unsafe block allows the dereferencing of raw pointers
    //  Creating a pointer does no harm; 
    // it’s only when we try to access the value that it points at that we might end up dealing with an invalid value.
    unsafe {
        println!("r1: {:?}, r2: {:?}", *r1, *r2);       // uncomment to see the error
    }
}
