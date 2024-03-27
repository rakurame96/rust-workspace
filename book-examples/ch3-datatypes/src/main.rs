use std::io;

fn main() {
    let integer_lit:u16 = 256u16;
    println!("integer_lit = {}", integer_lit);
    
    let x = 2.0; // f64 type
    println!("value of x : {}", x);
    
    let y : f32 = 2.1;  // f32 type
    println!("value of y : {}", y);

    // addition
    let sum = 5 + 55;
    println!("Sum: {sum}");

    //subtraction
    let sub = 55 - 50;
    println!("Sub: {sub}");

    //multiplication
    let multiply = 55 * 59;
    println!("multiply: {multiply}");

    // division
    let division = 800 / 50;
    println!("division : {division}");

    // quotient
    let quotient = 56.7 / 32.2;
    println!("quotient : {quotient}");

    // truncated
    let truncated = -5 / 3;
    println!("truncated : {truncated}");

    // boolean
    let boolean : bool = true;
    println!("boolean : {}", boolean);

    // character
    let character = "D7FF";
    println!("character : {}", character);

    // array
    let a = [1, 2, 3, 4, 5];
    
    println!("Please enter an array index.");
    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
   
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
   
    println!(
    "The value of the element at index {index} is: {element}"
    );
}
