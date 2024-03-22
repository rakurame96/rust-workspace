use std::fmt::Display;

fn main() {
    let numbers:[u32; 5] = [1, 2, 3, 4, 5];     // Array of type u32 each value is defined

    #[allow(unused_variables)]
    let numbers_1:[u32; 5] = [2; 5];            // All array elements will assume the value 2

    // to print the array, need to use the for loop
    for number in numbers {
        print!("{} ", number);
    }
    print!("\n");

    println!("{:?}", numbers_1);
    
    #[allow(unused_assignments)]
    /* array creation with automatic type and size inference */
    let mut my_numbers = [0;5];
    
    // let another_array = [10, 20, 30]; /* error line */
    let another_array = [10, 20, 30, 40, 50]; /* No error */    
    my_numbers = another_array;  // This would cause a compile-time error due to mismatched types

    println!("{:?}", my_numbers);

    // Array creation with explicit type and size
    let weekdays: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    
    // Accessing elements using indexing
    println!("The third weekday: {}", weekdays[2]); 

    // printing all elements using debug trait
    println!("Weekdays: {:?}", weekdays);

    // printing all elements using display trait
    println!("Weekdays: {:#?}", weekdays);
}
