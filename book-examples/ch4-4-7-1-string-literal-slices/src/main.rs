use std::ptr::read_unaligned;

// this program takes a string of words separated by spaces and returns the first word it finds in that string.
// if the function doesn't find a space in that string, the whole string must be one word, so entir string should be returned.
fn main() {
    // let mut s = String::from("hello world!");
    let s = "hello world!";
    println!("string s: {s}");

    // passes entire string as input to the first_word function
    let word = first_word(&s[..]);
    println!("word : {}", word);

    // because string literals are string slices already
    // this works too, without the slice syntax
    let word = first_word(s);
    println!("word : {}", word);
}

// below line can be rewritten as fn first_word(some_string: &Str) -> &str {
// as it allows to use same function for both STRING & String literals
fn first_word(some_string: &str) -> &str {
    // as_bytes() method converts the string to bytes format
    // bytes are the collections
    let bytes = some_string.as_bytes();
    println!("bytes : {:?}", bytes);
    
    // iterator to iterate over the array of bytes using iter method
    // iter method returns each element in a collections and that enumerate wraps the results of iter and returns each element as part of a tuple
    // return of iter is tuple (index, reference_to_element)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {    // here, I selected b' ' as space. space is present after 5th character.
        // if item == b'd' {    // here, I selected the b'd', then it will iterate until it reaches the desired byte 'd'
            return &some_string[0..i];
        }
    }

    // returns string as output
    &some_string[..]
}
