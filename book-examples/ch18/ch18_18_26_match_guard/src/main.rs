fn main() {
    let num = Some(4);
    match num {
        // A match guard is an additional if condition, specified after the pattern in a match arm that must also match for that arm to be chosen. 
        // Match guards are useful for expressing more complex ideas than a pattern alone allows
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
}
