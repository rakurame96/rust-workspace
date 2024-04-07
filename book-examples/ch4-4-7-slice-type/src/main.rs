fn main() {
    let mut s = String::from("hello world!");

    let word = first_word(&s);  // word will get the value 5

    s.clear();          // this empties the string, making it equal to "" or null.

    // word still has the value 5 here, but there's no more string that we could meaningfully use the value 5 with
    // word is now totally invalid!
}

fn first_word(some_string: &String) -> usize {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    some_string.len()
}
