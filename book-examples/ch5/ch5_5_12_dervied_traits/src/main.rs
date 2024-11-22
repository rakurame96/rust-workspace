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

    // prints everthnig in a single line. could be difficult to interpret its meaning
    println!("rect1 is {:?}", rect1);

    // formats the display message, and all fields are listed one by one not on single line
    println!("rect1 is {:#?}", rect1);

    // another method is using !dbg method
    dbg!(&rect1);

    println!(
    "The area of the rectangle is {} square pixels.",
    area(&rect1)
    );

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}