use std::cell::RefCell;

struct MyStruct {
    data: RefCell<i32>,
}

impl MyStruct {
    fn new(value: i32) -> Self {
        MyStruct {
            data: RefCell::new(value),
        }
    }

    fn increment(&self) {
        // This function can mutate `data` even though `self` is immutable.
        let mut data = self.data.borrow_mut();
        *data += 1;
    }

    fn get(&self) -> i32 {
        *self.data.borrow()  // Returns an immutable reference to the data
    }
}

fn main() {
    let my_struct = MyStruct::new(10);

    // Even though `my_struct` is immutable, we can modify `data` inside it.
    my_struct.increment();
    my_struct.increment();

    println!("Value: {}", my_struct.get());  // Prints "Value: 12"
}
