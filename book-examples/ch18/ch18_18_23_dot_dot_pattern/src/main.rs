#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };

    //.. pattern ignores any parts of a value that we haven’t explicitly matched in the rest of the pattern. 
    match origin {
        // This is quicker than having to list y: _ and z: _
        // particularly when we’re working with structs that have lots of fields in situations where only one or two fields are relevant
        Point { x, .. } => println!("x is {x}"),
    }
}
