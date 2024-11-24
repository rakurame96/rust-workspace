#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 5.5, y: 10.5};
    // let wont_work = Point {x: 5, y: 10.5};  // mismatched types for the same <T> type

    println!("integer x = {}, y = {}", integer.x, integer.y);
    println!("float x = {}, y = {}", float.x, float.y);
}
