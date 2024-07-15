use std::fmt::Display;

#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    #[allow(dead_code)]
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    #[allow(dead_code)]
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest number is x = {}", self.x);
        } else {
            println!("the largest number is y = {}", self.y);
        }
    }
}