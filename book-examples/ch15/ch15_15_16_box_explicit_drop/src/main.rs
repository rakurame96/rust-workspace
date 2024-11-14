struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    // not allowed explicit drop as rust also automatically cleans up the memory when it got out of scope. this will result in double free
    // c.drop();  // compile error

    println!("CustomSmartPointer Created.");
    // this function is from std::mem::drop
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    #[allow(unused)]
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
}
