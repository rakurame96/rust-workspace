use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Hashmap:{:?}", scores);

    scores.insert(String::from("Green"), 10);
    scores.insert(String::from("Red"), 50);

    println!("Hashmap:{:?}", scores);
}
