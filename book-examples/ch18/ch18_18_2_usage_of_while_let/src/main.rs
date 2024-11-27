fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    // popped value from the stack vector is stored in a variable called "top"
    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}
