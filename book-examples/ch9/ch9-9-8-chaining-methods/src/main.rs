use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // Listing 9-8: Chaining method calls after the ? operator
    println!("Listing 9-8: Chaining method calls after the ? operator");

    println!("read from username file : {:?}", read_username_from_file());
    
    // using match expressing in the calling code
    let res = read_username_from_file();
    /* method 1 */
    // match res {
    //     Ok(username) => println!("username: {}", username),
    //     Err(e) => println!("Error: {}", e),
    // };

    /* method 2 */
    match res {
        Ok(username) => println!("username: {}", username),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("file not found {}", e)
            },
            other_error => {
                println!("other errors {}", other_error)
            }
        } 
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}