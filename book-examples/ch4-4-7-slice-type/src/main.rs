// this program takes a string of words separated by spaces and returns the first word it finds in that string.
// if the function doesn't find a space in that string, the whole string must be one word, so entir string should be returned.
fn main() {
    let mut s = String::from("hello world!");

    println!("string s: {s}");

    let word = first_word(&s);  // word will get the value 5

    println!("word : {word}");

    s.clear();          // this empties the string, making it equal to "" or null.

    println!("string s: {s}");
    // word still has the value 5 here, but there's no more string that we could meaningfully use the value 5 with
    // word is now totally invalid!
}

fn first_word(some_string: &String) -> usize {
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
            return i;
        }
    }

    some_string.len()
}
