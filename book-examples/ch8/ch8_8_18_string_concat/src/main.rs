fn main() {
    // Listing 8-17: Adding one character to a String value using push
    let mut s = String::from("lo");
    s.push('l');
    println!("s : {}", s);

    // Listing 8-18: Using the + operator to combine two String values into a new String value
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("s3 : {}", s3);

    // error
    // println!("s1 : {}", s1);
}
