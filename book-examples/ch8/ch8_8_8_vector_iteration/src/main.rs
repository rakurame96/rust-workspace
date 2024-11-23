//
// To change the value that the mutable reference refers to, 
// we have to use the * dereference operator to get to the value in i before we can use the += operator
fn main() {

    // iterating over immutable vector references
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // iterating over mutable vector references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;

        println!("vector v = {}", i);
    }
}
