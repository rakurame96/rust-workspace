use std::env;
use std::fs::File;

#[allow(unused_variables)]
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // Listing 9-3: opening a file
    // let greeting_file_result = File::open("hello.txt");

    // println! statement will print the below error
    // result of greeting_file : Err(Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." })
    // println!("result of greeting_file : {:?}", greeting_file_result);


    // Listing 9-4: Using Result variants using match expressions
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };

    // println!("greeting_file : {:?}", greeting_file);
}
