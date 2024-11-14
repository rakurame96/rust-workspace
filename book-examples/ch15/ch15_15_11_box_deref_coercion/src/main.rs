use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    let z = MyBox::new(String::from("Rakuram"));

    assert_eq!(5, x);
    // *y is equal to *(y.deref())
    assert_eq!(5, *y);

    // If Rust didn’t implement deref coercion, we would have to write the code --> hello(&(*m)[..]);
    // The (*m) dereferences the MyBox<String> into a String. 
    // Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello. 
    // This code without deref coercions is harder to read, write, and understand with all of these symbols involved. 
    // Deref coercion allows Rust to handle these conversions for us automatically.
    hello(&z);
}
