/* Tuples */

fn circle_properties(radius: f64) -> (f64, f64){
    let area = std::f64::consts::PI * radius.powi(2);
    let circumference = 2.0 * std::f64::consts::PI * radius;
    (area, circumference)
}

fn circle_properties_1(radius: f64) -> f64{
    let area = std::f64::consts::PI * radius.powi(2);
    // let circumference = 2.0 * std::f64::consts::PI * radius;
    area
}

fn main(){

    /*
     * Here, my_tuple is a tuple that holds an integer, a floating-point number, and a string.
     * Notice how we can store different types of data in the same tuple - that’s one of the things that make tuples so versatile!
     */
    let my_tuple: (u32, f32, String) = (10, 3.14, "Hello".to_string());

    println!("First element (integer): {}", my_tuple.0);
    println!("Second element (float): {}", my_tuple.1);
    println!("Third element (string): {}", my_tuple.2);

    /*
     * destructuring of the tuples
     */
    let (x, y, z) = my_tuple;
    println!("Destructured values: x = {}, y = {}, z = {}", x, y, z);

    let (rad, cir) = circle_properties(5.0);
    println!("Circle Properties : Radius = {}, Circumference = {}", rad, cir);

    let rad = circle_properties_1(6.0);
    println!("Circle properties : Radius = {}", rad);

    // Tuple elements can be mutated:
    let mut another_tuple = (1, vec![2, 3, 4]); 
    another_tuple.1.push(5); // Modifying the vector within the tuple
    
    println!("Modified tuple: {:?}", another_tuple);
}