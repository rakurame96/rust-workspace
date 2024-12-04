fn main() {
    let x = 4;
    let y = false;
    match x {
        // The match condition states that the arm only matches if the value of x is equal to 4, 5, or 6 and if y is true.
        // here for y is mentioned alone means, it will imply as y == true
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
