fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Correct location: ({x}, {y})")
}

fn main() {
    let point: (i32, i32) = (3, 7);
    print_coordinates(&point);
}
