use std::error::Error;
use std::fs::File;

#[allow(unused)]
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    
    Ok(())
}
