use std::io::{self, Read};
use std::fs::File;

fn main() {
    // Listing 9-7: A function that returns errors to the calling code using the ? operator
    println!("Listing 9-7: A function that returns errors to the calling code using the ? operator");

    println!("read from username file : {:?}", read_username_from_file());

    let res = read_username_from_file();
    match res {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("Error: {}", e),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");

    // ? returns the error to the calling code without need for match expression
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username), 
    //     Err(e) => Err(e),
    // }
    username_file.read_to_string(&mut username)?;
    Ok(username)
}