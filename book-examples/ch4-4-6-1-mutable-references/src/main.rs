// mutable references have one big restriction: 
// if you have a mutable reference to a value, you can have no other references to that value
// this code attempts to create two mutable references to s will fail
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{r1}");

    // cannot borrow mutable reference to more than 1 
    let r2 = &mut s;
    // println!("{r1} & {r2}");  // error
    println!("{r2}");  // no error

    // combining both mutable and immutable reference is not allowed in rust.
    // this is to avoid the data race condition at the later stage of the code

    let mut s1 = String::from("hello");

    let rr1 = &s1;  // no problem - immutable reference
    let rr2 = &s1;  // no problem
    println!("{rr1} & {rr2}");  // variables rr1 & rr2 will not be used after this point

    let rr3 = &mut s1;  // problem

    // println!("{rr1}, {rr2}, & {rr3}");  // error case

    println!("{rr3}");  // no error case

    // once the variable is used as mutable, then it should not refer to immutable reference
    // println!("{rr1}");
}
