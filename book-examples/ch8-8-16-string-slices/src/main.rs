fn main() {
    // Appending a string slice to a String using the push_str method
    let mut s = String::from("foo");
    s.push_str("bar");


    // Using a string slice after appending its contents to a String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
}
