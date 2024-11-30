fn main() {
    // syntax _x still binds the value to the variable
    // but _ doesn’t bind at all.
    let _x = 5;
    let y = 10;

    println!("value of _x: {}, y: {}", _x, y);
}
