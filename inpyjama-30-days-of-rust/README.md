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

#### to update the *Crate* to get a New Version
```shell
cargo update
```

#### to read documentation about the installed crates
```shell
cargo doc --open
```

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
* Const can be defined in both local and global scope. 
* Const must be annotated with the datatype unlike the other variables
* Rust suggested way of creating constants is, all uppercase with underscores between words
```rust
// consts GLOBAL_CONST = 180;     // compile error as constants always needs to be annotated with the datatype
const GLOBAL_CONST: u32 = 180;  
```

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

> **_If-else case example from rustlings_**
```rust
pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {
        a
    } else {
        b
    }
}```

```rust
pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz"{
        "bar" 
    } else {
        "baz"
    }
}
```

> **_Explanation about `&'static str`_**
`pub fn animal_habitat(animal: &str) -> &'static str`
First of all, &'static str is a type which consists of 3 parts - &, 'static, str.

& means this type is a reference, borrowed from a value owned by someone else. Every references has associated its lifetime and the Rust compiler proves that the usage of the reference never exceed its lifetime. Safe Rust cannot touches dangling reference even by accident.

'a is a lifetime notation named a, and 'static is a special one which represents the lifetime of the process itself. It ends when the process terminates so it can be practically considered endless. &'static means a reference that cannot be dangling.

str is a type which represents a sequence of bytes which also is valid UTF-8 sequence. It's size cannot be determined statically as there isn't single "length" for every possible text data. In Rust we call such types DST(Dynamically Sized Type) and they cannot be placed on the stack without some indirection types like Box<T>, &T etc.

As a conclusion, &'static str is a reference to the UTF-8 encoded variable length of byte sequence, which is valid for the entire lifetime of the process. But how can we obtain such type? Normally it's from the string literals, which are embeded directly on the executable binary(like .rodata section of the elf file) and loaded to the read-only section of the memory by the OS before execution.

> **_Tuples_**
Tuples are a simple yet powerful feature that allows us to group together multiple values of different types. They are useful when you want to return multiple values from a function or when you want to group related data together.

In Rust, once a tuple is created, its size and the types of its elements cannot be changed. This might give the impression that tuples are immutable, but that's not true. Elements of a tuple can be mutated.

> **_Arrays_**
Arrays are a fundamental data structure that allow us to store multiple values of the same type in a single variable. They’re incredibly useful for organizing data and making our code more efficient and readable.
* Syntax: `let variable_name : [data_type; size]`
* specify the type of data the array will hold, and the size of the array
* Example: `let my_numbers: [u32; 5] = [1, 2, 3, 4, 5];`
* Accessing elements in an array is straightforward. We use indexing, just like in most other programming languages.
* Example: `println!("The first number: {}", my_numbers[0]);`

* **NOTE** Arrays in rust are bound-checked. This means that if you try to access or modify an element at an index that doesn’t exist, Rust will throw a compile-time error

> **_Macros in Rust_**
* `#[warn(unused_variables)]` - This option is by default ON, informs about the dead code
* `#[allow(unused_variables)]` - suppress the dead code warning
* `#[warn(unused_assignments)]` - This option is by default ON, informs about any unused assignment
* `#[allow(unused_assignments)]` - allows unused assignment
* `#[derive(Debug)]` - To bring-in the debug trait for printing
* `#[warn(unused_imports)]` - This option is ON by default, informs about the unused imports
* `#[allow(unused_imports)]` - suppress the unused imports warning

# Interesting Articles to read
* Author of this below site : [Amos Wenger](https://github.com/fasterthanlime)
* https://fasterthanli.me/articles/a-half-hour-to-learn-rust

# Reference material:
1. [Why Rust](https://inpyjama.com/day0-why-rust/)
2. [Day 1: Setting Up the Environment](https://inpyjama.com/day1-setting-up-the-environment/)
3. [Day 2: Hello, Rust](https://inpyjama.com/day2-hello-rust/)
4. [Day 3: Prints in Rust](https://inpyjama.com/day3-printing/)
5. [Day 4: Printing Custom types](https://inpyjama.com/day4-debug-display/)
6. [Day 5: Variables](https://inpyjama.com/day-5-data-types-in-rust/)
7. [Day 6: Variables Continued](https://inpyjama.com/day6-variables-contd/)
8. [Day 7: Rust hate Implicit](https://inpyjama.com/day7-rust-hate-implicits/)
9. [Day 8: Tuples](https://inpyjama.com/day-8-tuples/)
10. [Day 9: Arrays](https://inpyjama.com/day-9-arrays/)
11. [Day 10: Heap vs Stack](https://inpyjama.com/day-10-heap-vs-stack/)