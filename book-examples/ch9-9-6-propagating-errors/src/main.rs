use std::io::{self, Read};
use std::fs::File;

fn main() {
    println!("read from username file : {:?}", read_username_from_file());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // no semicolon because it returns the String & Error to the calling function
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username), 
        Err(e) => Err(e),
    }
}