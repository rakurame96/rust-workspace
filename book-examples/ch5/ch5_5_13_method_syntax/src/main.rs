// Refactoring with Structs: Adding More Meaning
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
   }
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // another method is using !dbg method
    dbg!(&rect1);

    println!(
    "The area of the rectangle is {} square pixels and perimeter of the rectangle is {} square pixels.",
    rect1.area(), rect1.perimeter()
    );

    let scale = 100;
    let rect2 = Rectangle {
        width: 20,
        height: (scale * 2),
    };
    
    println!("perimeter of rect2: {}", rect2.perimeter());
}

impl Rectangle{
    // fn area(rectangle: &Rectangle) -> u32 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter (&self) -> u32 {
        2 * (self.height * self.width)
    }
}

impl Rectangle {
    
}