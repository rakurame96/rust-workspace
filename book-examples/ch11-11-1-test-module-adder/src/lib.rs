pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_does_work() {
        let result = add(0, 5);
        assert_eq!(result, 5);
        assert_eq!(result, 5, "we are testing addition with {} and {}", result, 5);

    }

    // #[test]
    // fn make_test_fail() {
    //     panic!("make test fail intentionally");
    // }
}
