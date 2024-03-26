fn main(){
    let pi:f64 = 3.14;

    println!("pi: {}", pi);

    // uncomment the below line and then compile
    // let whole_pi:u32 = pi; // error

    let whole_pi:u32 = pi as u32;   // explicit type conversions

    println!("Whole part of pi: {}", whole_pi);

    // another way of printing 
    println!("pi: {}", pi as u32);
}