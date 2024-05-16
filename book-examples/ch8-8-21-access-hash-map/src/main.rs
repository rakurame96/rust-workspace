use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Hashmap:{:?}", scores);

    scores.insert(String::from("Green"), 10);
    scores.insert(String::from("Red"), 50);

    println!("Hashmap:{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);   

    println!("Score: {}", score);

    // We can iterate over each key-value pair in a hash map in a similar manner as we do with vectors, using a for loop:
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
