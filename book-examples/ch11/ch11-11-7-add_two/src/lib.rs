pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_three(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_doesnot_work() {
        let result = add_three(1);
        assert_ne!(result, 4);
    }
}
