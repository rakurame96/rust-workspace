#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 5.5, y: 10.5};

    println!("integer x = {}, y = {}", integer.x, integer.y);
    println!("float x = {}, y = {}", float.x, float.y);
}
