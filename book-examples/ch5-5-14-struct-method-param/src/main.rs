struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_struct: &Rectangle) -> bool {
        (self.width > other_struct.width) && (self.height > other_struct.height)
    }

    fn multiple(&self, factor: u32) -> u32 {
        self.width*factor + self.height*factor
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 55,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 15,
    };

    println!("Area of rect1 : {} & rect2 : {}", rect1.area(), rect2.area());
    println!("Can rect1 hold rect2 : {}", rect1.can_hold(&rect2));
    println!("factor of 2 : {}", rect1.multiple(2));
}
