fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("the length of {s1} is {len}.");

    let mut s = String::from("hello");

    // below line will not work. as the string is not mutable
    // change(&s);
    
    // modified the variable s as a mutable - mutable references
    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// below line will throw the error
// fn change(some_string: &String) {
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}