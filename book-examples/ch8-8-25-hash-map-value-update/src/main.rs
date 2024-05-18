use std::collections::HashMap;

fn main() {
    // Listing 8-23: Replacing a value stored with a particular key
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Listing 8-24: Using the entry method to only insert if the key does not already have a value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Listing 8-25: Counting occurrences of words using a hash map that stores words and counts
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        println!("Before: -> word : {} & count : {}", word, count);
        // *count means updating the reference
        *count += 1;
        println!("After: -> word : {} & count : {}", word, count);
    }
    println!("{:?}", map);
}
