fn main() {
    let x = 5;
    
    match x {
        // 1..5 can also expressed as 1 | 2 | 3 | 4 | 5

        // 1..5 is exclusive of 5, including 1, 2, 3, and 4.
        // 1..=5 is inclusive of 5, including 1, 2, 3, 4, and 5.
        1..5 => println!("one through 5"),
        _ => println!("anything else"),
    }
}
