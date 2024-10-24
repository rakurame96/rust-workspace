struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);

    // unit-like structs
    let subject = AlwaysEqual;
    println!("subject: {:?}", subject);
}
