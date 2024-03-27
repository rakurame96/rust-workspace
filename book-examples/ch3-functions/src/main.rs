fn main() {
    println!("Hello, world!");
    println!("print from the main function");
    let ret = another_function(5);
    println!("Return : {}", ret);
}

fn another_function(num: u32) -> u32 {
    println!("print from another function");

    return num;     // valid return expression
    // num;         // error
    // num          // valid return expression
}