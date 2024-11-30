// whatever value is being matched will get ignored
fn foo(_:i32, y:i32) {
    println!("this code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);
}
