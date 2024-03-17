#### Below command will create the GIT repo with initialization
```shell
cargo new <package_name>
```

#### Below command will create the GIT repo without initialization
```shell
cargo new <package_name> --vcs none
```

#### to build
```shell
cargo build
```

#### to run
```shell
cargo run
```

#### for debug information
```shell
rustc --explain <ErrorCode>
```
Example: `rustc --explain E0277`

#### to compile single file
```shell
rustc <rust ssource file name>
```
Example: `rustc main.rs`

> **_NOTE: Ruslings Commands_**
Commands available to you in watch mode:
  * `hint`   - prints the current exercise's hint
  * `clear`  - clears the screen
  * `quit`   - quits watch mode
  * `!<cmd>` - executes a command, like `!rustc --explain E0381`
  * `help`   - displays this help message

> **_NOTE: Mutable and Immutable_**
Mutable objects are those that allow you to change their value or data in place without affecting the object's identity. In contrast, immutable objects don't allow this kind of operation. You'll just have the option of creating new objects of the same type with different values.

> **_NOTE: Shadowing_**
```rust
fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
```
In the above program, variable `number` is considered as string and the same variable `number` reused as integer. Only thing we need to do is, use the `let` keyword. In this case, we cannot use `mut` keyword. As it denotes that the variable is mutable and value can be modified. In general, variables created using `let` are immutable.

> **_ NOTE: Function in Rust _**
```rust 
fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}
```
Return of the function is mentioned as `fn sale_price(price: i32) -> i32`, operator `->` denotes the function is returning something and type of data the function is returning mentioned `-> <datatype>`. Example: `fn sale_price(price: i32) -> i32`

For function, where it is returing data back to the caller, then in the particular line of code we should not use `semicolon` - `;`

# Reference material:
1. [Why Rust](https://inpyjama.com/day0-why-rust/)
2. [Day 1: Setting Up the Environment](https://inpyjama.com/day1-setting-up-the-environment/)
3. [Day 2: Hello, Rust](https://inpyjama.com/day2-hello-rust/)
4. [Day 3: Prints in Rust](https://inpyjama.com/day3-printing/)
5. [Day 4: Printing Custom types](https://inpyjama.com/day4-debug-display/)
6. [Day 5: Variables](https://inpyjama.com/day-5-data-types-in-rust/)