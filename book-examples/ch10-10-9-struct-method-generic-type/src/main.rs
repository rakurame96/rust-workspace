#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

struct Point1<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    // here &self represents, it accepts the struct of same type where the instance is created
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> Point1<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10};

    println!("p.x : {}", p.x());

    let p1 = Point1 { x: 55, y: 5.55};

    println!("p1.x : {}, p1.y : {}", p1.x(), p1.y());
}