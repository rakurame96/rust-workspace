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

#[macro_export]
macro_rules! vec_macro {
    ($( $x:expr ),* ) => {
        let mut v = Vec::new();
        $(v.push($x); )*
        v
    };
}

#[macro_export]
macro_rules! vec_macro_comma {
    ($( $x:expr ),* ,) => {
        let mut v = Vec::new();
        $(v.push($x); )*
        v
    };
}

#[macro_export]
macro_rules! vec_macro_comma_plus {
    ($( $x:expr ),+ ,) => {
        let mut v = Vec::new();
        vec![$( $x ),*]
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

    #[test]
    fn vec_test() {
        vec_macro!(5, 6);
    }

    #[test]
    fn vec_test_with_comma() {
        vec_macro_comma!(5, 6, );
    }

    #[test]
    fn vec_test_with_comma_plus() {
        vec_macro_comma!(5, 6, 7,);
    }
}
