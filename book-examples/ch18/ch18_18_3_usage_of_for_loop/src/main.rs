fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // We adapt an iterator using the enumerate method so it produces a value and the index for that value, placed into a tuple. 
    // The first value produced is the tuple (0, 'a'). 
    // When this value is matched to the pattern (index, value), `index` will be 0 and `value` will be 'a', printing the first line of the output.
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // iter() only takes the one value at a time and prints it
    // enumerate() can be used on top of the iter() not as a standalone
    println!("Only using the iter() function in the vector");
    for value in v.iter() {
        println!("{value}");
    }
}
