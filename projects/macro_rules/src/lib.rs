use std::io::Write;

macro_rules! my_println {
    ($x: expr) => {
        let _ = writeln!(std::io::stdout(), "{}", $x);
        };
}

macro_rules! my_println_with_args {
    ($($args:tt)*) => {
        let _ = writeln!(std::io::stdout(), "{}", format_args!($($args)*));
        };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let name = "Rakuram";
        // assert_eq!(result, 4);
        my_println!("Hello!");
        my_println_with_args!("Hello, {}!", name);
    }

    // #[test]
    // fn it_doesnot_works() {
        // let result = square!(2);
        // assert_eq!(result, 6);
    // }
}
