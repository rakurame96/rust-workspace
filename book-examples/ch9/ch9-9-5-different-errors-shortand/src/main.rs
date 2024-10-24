use std::fs::File;
use std::io::ErrorKind;

#[allow(unused_variables)]
fn main() {
    let greeting_file_result = File::open("hello.txt");

    /*
    // using match expression
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!(
                        "Problem in creating the file: {:?}", e
                    ),
                }
            }
            other_error => {
                panic!("Porblem in opening the file: {:?}", other_error);
            }
        },
    };
    */
    
    // using closures instead of match
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        }
        else {
            panic!("Problem in opening the file: {:?}", error);
        }
    });
}


