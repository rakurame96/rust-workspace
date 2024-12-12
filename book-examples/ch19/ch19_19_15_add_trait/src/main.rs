use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let mtrs = Meters (5);
    println!("mtrs: {:?}", mtrs);

    let millimtrs = Millimeters (1000);
    println!("millimtrs: {:?}", millimtrs);

    let result = millimtrs.add(mtrs);
    println!("Result: {:?}", result);
}
