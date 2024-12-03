fn main() {
    let numbers = (2, 4, 8, 16, 32);
    
    match numbers {
        // the first and last values are matched with first and last. 
        // The .. will match and ignore everything in the middle
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
