use std::fs;
use std::io;

fn main() {
    // Listing 9-9: Using fs::read_to_string instead of opening and then reading the file

    println!("Listing 9-9: Using fs::read_to_string instead of opening and then reading the file");

    let res = read_username_from_file();
    /* method 1 */
    match res {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("Error: {}", e),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}