struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point {
        x: 0,
        y: 8,
    };

    // x and y is a variable and it will match with the Struct variable. No need of creating new variables with unique name again
    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(8, y);
}
