struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point {
        x: 0,
        y: 8,
    };

    match p {
        Point {x, y: 0} => println!("on the x axis at {x}"),
        Point {x: 0, y} => println!("on the y axis at {y}"),
        Point {x, y} => println!("on neither axis: ({x}, {y})"),
    }
}
