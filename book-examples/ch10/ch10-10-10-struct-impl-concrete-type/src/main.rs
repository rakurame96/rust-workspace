struct Point<T> {
    x: T, 
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2).sqrt()
    }
}

fn main() {
    let x_p: Point<f32> = Point {x: 8.1, y: 9.1};

    println!("x_p.x : {}, x_p.y : {}", x_p.x, x_p.y);

    println!("distance_from_origin: {}", x_p.distance_from_origin());

    let x_p1 = Point { x: 5, y: 10};

    println!("x_p1.x : {}, x_p1.y : {}", x_p1.x, x_p1.y);

    // below line will throw the error as the <T> is not of <f32> type
    // println!("distance_from_origin: {}", x_p1.distance_from_origin());
}
