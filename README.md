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

> **_NOTE: Function in Rust_**
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

> **_NOTE: Why unsafe keyword is necessary in the above situation_**
Looking at the error, it complains that using mutable statics is unsafe and needs an unsafe block. It also mentions the potential for data races if increment_counter is called from multiple threads. Rust isn't being picky – it's absolutely right! Why? Because multiple threads could potentially call increment_counter simultaneously, leading to a data race – a situation where multiple threads access the same memory location without proper synchronization. This can corrupt the value of COUNTER.

This is how Rust ensures memory safety – by refusing to compile code that could lead to trouble. However, there might be times when you absolutely need a mutable static. The compiler leaves us a hint: mutable statics require an unsafe block. Think of the unsafe keyword as a backdoor for special cases where you need to bypass Rust's usual safety checks.

##### Constants and Statics
**Consts** are essentially fixed values known at compile time. They must have a type annotation and are perfect for situations like array sizes or mathematical constants. They behave similarly to #define macros in C, but are type-safe and prevent unexpected behavior

**Statics** are similar to const, but they have a fixed memory location throughout the program's execution. This makes them useful for global variables that need to persist, like configuration settings or file handles. However, unlike const, statics can't be used in all contexts (like within functions) due to their global nature.

While statics can be declared as mutable (using mut), Rust discourages this practice and considers it unsafe. Let's see this with a quick experiment

###### Example for Statics
```rust
static mut COUNTER: u32 = 0; // Mutable static variable

fn increment_counter() {
    COUNTER += 1; // This line is unsafe!
}

fn main() {
    increment_counter();
    println!("{}", COUNTER); // Will this always print 1?
}
```

> **_Type Conversions in Rust_** 
Rust prioritizes safety and predictability, requiring developers to be explicit about type conversions. So, implicit type conversions is not allowed in Rust and if user tries to perform implicit conversion, compilation won't happen.

User need to explicitly mention the type conversion using `as` keyword and the target type. 

The Rust compiler won't let this slide. It complains about a type mismatch because it refuses to implicitly convert from a floating-point type to an integer. While Rust prefers explicit type conversions, it provides the `as` operator for those intentional changes. The `as` operator offers a controlled way to convert between compatible numeric types.

By making conversions explicit, Rust forces you to confront these potential problems head-on. Second, explicit conversions enhance code readability by clearly signifying where type changes are happening and what your intentions are.

```C
#include <stdio.h>

int main() {
    double pi = 3.14;
    int whole_pi = pi;  // Implicit conversion from double to int
    printf("Whole part of pi: %d\n", whole_pi);  // Output: 3
}
```

```rust
fn main() {
    let pi:f64 = 3.14; 
    let whole_pi:u32 = pi as u32;  
    println!("Whole part of pi: {}", whole_pi);
}
```

# Reference material:
1. [Why Rust](https://inpyjama.com/day0-why-rust/)
2. [Day 1: Setting Up the Environment](https://inpyjama.com/day1-setting-up-the-environment/)
3. [Day 2: Hello, Rust](https://inpyjama.com/day2-hello-rust/)
4. [Day 3: Prints in Rust](https://inpyjama.com/day3-printing/)
5. [Day 4: Printing Custom types](https://inpyjama.com/day4-debug-display/)
6. [Day 5: Variables](https://inpyjama.com/day-5-data-types-in-rust/)
7. [Day 6: Variables Continued](https://inpyjama.com/day6-variables-contd/)
8. [Day 7: Rust hate Implicit](https://inpyjama.com/day7-rust-hate-implicits/)