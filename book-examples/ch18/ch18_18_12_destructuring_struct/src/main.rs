struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point {
        x: 0,
        y: 8,
    };

    // value stored in x = 0 and a also refers the same (kind of mirroring)
    let Point {
        x: a, 
        y: b} = p;

    assert_eq!(0, a);
    assert_eq!(8, b);
}
