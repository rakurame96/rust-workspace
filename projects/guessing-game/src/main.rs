/// std => standard library
/// io => input/output library 

/// This is necessary to obtain user input and then print the result as output
/// Then we need to bring io input/output library (dependencies) into scope
/// 
/// Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude
/// 
/// Verbose => excessive or unnecessary use of words in code, documentation, or communication
use std::io;    

/// main() function is the entry point for all rust programs
fn main() {
    println!("Guesss the number!");

    println!("Please input your guess.");

    // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text
    // The :: syntax in the ::new line indicates that new is an associated function of the String type. 
    // An associated function is a function that’s implemented on a type, in this case String.
    
    let mut guess = String::new();

    // below line is also valid input for user convenience it is splitted into 2 lines
    /*
     * io::stdin().read_line(&mut guess).expect("Failed to read line");
     */

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // so, contents stored in guess will not be overwritten upon calling the `.read_line` function for the second time
    // we can also invoke this function by using below line
    // `io::stdin::read_line` - also valid input
    println!("Please input your guess.");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);

}
