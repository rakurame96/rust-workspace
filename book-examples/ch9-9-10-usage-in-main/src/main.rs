#[allow(unused)]
use std::fs::File;

fn main() {
    // Listing 9-10: Attempting to use the ? in the main function that returns () won’t compile.
    println!("Listing 9-10: Attempting to use the ? in the main function that returns () won’t compile.");

    // uncomment and compile the code
    // let greeting_file = File::open("hello.txt")?;

    // usage of ? within the main function will fail. 
    // The ? operator follows the Result value returned by File::open, but this main function has the return type of (), not Result.
    // Error points that, usage of ? operator in a function that returns Result, Option or another type that implements FromResidual
}
