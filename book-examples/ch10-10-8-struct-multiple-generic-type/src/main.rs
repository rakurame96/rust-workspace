#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 5.5, y: 10.5};
    let will_work = Point {x: 5, y: 10.5};  // mismatched types for the same <T> type

    println!("integer x = {}, y = {}", integer.x, integer.y);
    println!("float x = {}, y = {}", float.x, float.y);
    println!("will work x = {}, y = {}", will_work.x, will_work.y);
}
