#![deny(clippy::too_many_arguments)]

fn main() {
    println!("Hello, world!");

    test_lint(1, 2, 3, 4, 5, 6, 7, 8);
    test_string(16);
}

fn test_lint(arg1: u8, arg2: u8, arg3: u8, arg4: u8, arg5: u8, arg6: u8, arg7: u8, arg8: u8) {

}

#[allow(unused_variables)]
fn test_string(arg1: u16) {
    let x = "hello from lint-help";

    println!("{}", x);
}