#[macro_export]
macro_rules! own_assert_eq {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                println!("resultant value of comparison: {:?}", *left_val == *right_val);
                if !(*left_val == *right_val) {
                    panic!("assertion failed: `left == right` \
                            (left: {:?}, right: {:?})", left_val, right_val)
                }
            }
        }
        
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[should_panic]
    fn it_works() {
        own_assert_eq!(4, 4);
    }
}
