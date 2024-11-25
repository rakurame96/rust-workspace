#[allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(unused)]
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_can_hold_smaller() {
        let larger = Rectangle {
            width: 8, 
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!((larger.can_hold(&smaller)));
    }

    #[test]
    fn fn_smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8, 
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!((!smaller.can_hold(&larger)));
        assert_eq!(false, smaller.can_hold(&larger));       // both are valid statements
    }
}
