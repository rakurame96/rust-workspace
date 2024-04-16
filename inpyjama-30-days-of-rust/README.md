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
* `#[deny(overflowing_literals)]` - deny overflowing literals ON by default
* to suppress compiler warning on unused variables, prefix the variable name with an underscore: 
    * example: `_index` -> variables whose name start with underscore `_` compiler warning will be suppressed

> **_Datatypes_**
* Rust is a statically typed language
* Two types of datatype in rust
    * **scalar type**
        * it represents the single value. Rust has 4 primary scalar types
            * integers
                * signed
                * unsigned
            * floating-point numbers
                * two types
                    * f64 (default) - double precision
                    * f32 - single precision
                * all floating-point types are signed
            * booleans
            * characters

    * **compound type**
        * two compound datatypes
            * Tuple Type
                * A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
                * We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same
                * Example_1
                ```rust
                let tup: (u32, f32, char, u8) = (500, 6.4, "a", 1);

                let (a, b, c, d) = tup;     // pattern matching concept

                // what happens here is, 
                // a = 500, b = 6.4, c = 'a', d = 1
                // each variable binds to one tuple 
                ```
                * Example_2
                ```rust
                fn main() {
                    let x: (i32, f64, u8) = (500, 6.4, 1);
                    let five_hundred = x.0;     // first element with index[0] from the x assigned to the variable five_hundred
                    let six_point_four = x.1;
                    let one = x.2;
                }
                ```
            * Array Type
    * **String type**
        * Rust's char type is not limited to a single byte. Instead, it represents a single **Unicode Scalar Value**. Similar to Rust chars, Rust strings are not limited to ASCII chars and can represent **UTF-8**. This ensures your Rust programs can handle text from around the world!
        * Strings can have elements of varying sizes. Using the same size for all char representations would waste a lot of memory. For example, to represent the `🦀` Rust needs 4 bytes, but 'R' can be represented using a single byte.
        * This means that elements in a Rust string are not all of the same size, and we cannot access them using indexing directly, as rust does not know what the right index boundary should be.
        * to access individual elements in Rust, compiler suggests we should access the index using either `chars().nth()` or `bytes().nth()`.
        * Let's have a look at how strings are represented in memory. 
            * A String internally has three parts: 
                * a pointer to the data on the heap, 
                * the length of the string (how many bytes), and 
                * its capacity (the total allocated space on the heap). 
            * One of the things to notice is that Rust's **String type explicitly stores the length of the string**. This eliminates the need for **null termination** and prevents the **risk of reading invalid memory**.
        * to expand the string, we need to use `push_str()` method. 

> **_Functions_**
* Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.
* In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations 
in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. The compiler 
is also able to give more helpful error messages if it knows what types the function expects

**Statements & Expressions**
* Statements are instructions that perform some action and do not return a value.
* Expressions evaluate to a resultant value.

> **_Conditional_Statements_**
* if
* if-else

> **_Loops_**
* Different types of loops in Rust:
    * repeating code `loop` keyword
        * returning values from loops
        * loop labels to disambiguate between multiple loops
    * conditional loops with `while` 
    * looping through a collection through `for` loop
```rust
// loop syntax without loop labels
fn main() {
    loop {
        println!("Again");
    }
}
```
```rust
// loop syntax with loop labels
// labels help to break the loop by calling `break <loop_label>`
fn main() {
    'counting_up : loop {
        println!("counting_up");
    }
}
```
* we can break the loop with the help of loop label `break <loop_label`

> **_Import of libraries or dependencies_**
```rust
use std::cmp::min;

let least = min(7, 1); // this is 1
```
**alternate ways to import libraries**
```rust
// this works:
use std::cmp::min;
use std::cmp::max;

// this also works:
use std::cmp::{min, max};

// this also works!
use std::{cmp::min, cmp::max};
```

**Concept of Ownership in rust**
> The types covered previously (u8/i8, f8/16, etc,.) are of a known size, can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope.

**Shallow Copy and Deep Copy**
> A **shallow copy** creates a new object and inserts references to the elements of the original object. So, if you modify the original object, the changes will be reflected in the shallow copy. Shallow copying is faster and uses less memory because it only copies the references to objects, not the objects themselves. In some cases, having multiple references to the same object can be useful. For example, if multiple parts of your code need to interact with the same object, a shallow copy can be an excellent choice. However, Since the copied variables point to the same memory location, changes to the original object will affect the copied object and vice versa. This can lead to unexpected behavior if not handled carefully.

> A **deep copy**, on the other hand, creates a new object and recursively adds copies of the elements of the original object. Changes to the original object do not affect the deep copy. This is useful when you need to work with a copy of an object without affecting the original. However, as you can predict performing a deep copy takes a toll on both memory and cpu cycles. Deep copying is slower and uses more memory than shallow copying because it involves creating copies of all elements of the copied object. Implementing deep copy can be complex, especially for objects with nested or recursive data structures. You need to recursively make a copy of all the members of the structures.

**Garbage Collection**
> Python employs a mechanism known as **garbage collection** for automatic memory management. The garbage collector, a part of Python’s standard library, is engineered to reclaim memory that’s occupied by objects no longer in use by the program. The garbage collector in Python operates using a method called reference counting. Each object has a count that keeps track of the number of references to it. When an object’s reference count falls to zero, it becomes unreachable and is marked for garbage collection.

**References and Borrowing**
> A reference is like a pointer in that it's and address we can follow to access the data stored at that address; that data is owned by some other variable.
> Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

> **Note:** The opposite of referencing by using `&` is dereferencing, which is accomplished with the dereference operator `*`
```rust
// Example for referencing and the ownership
fn main() {
    let s1 = String::from("hello");

    // here, &s1 lets us create a reference that refers to the value of s1 but does not own it.

    // because it does not own it, the value points to will not be dropped when the reference stops being used.
    let len = calculate_length(&s1);

    println!("the length of {s1} is {len}.");
}

// s is a reference to a string
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, the string is not dropped.
```

**Mutable Referenes**
> It allows us to modify a borrowed value with just a few small tweaks that use.
```rust
fn main() {
    let mut s = String::from("hello");

    // below line will not work. as the string is not mutable
    // change(&s);
    
    // modified the variable s as a mutable - mutable references
    change(&mut s);
}

// below line will throw the error
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**Slice type**
> **Slice** let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
> `iter` method returns a tuple of 2 elements 
    > 1st element is `index`
    > 2nd element is `reference_to_the_element_itself`
    > example: `(i, &item)`
> **string slices** - is a reference to part of a string and it looks like below: `[starting_index..ending_index]`
```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

**Difference between `String` & `&str`**
> `String` is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.
> `str` is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory. Since the size is unknown, one can only handle it behind a pointer. This means that `str` most commonly appears as `&str`: a reference to some UTF-8 data, normally called a "string slice" or just a "slice". A slice is just a view onto some data, and that data can be anywhere, e.g.
    > **In static storage:** a string literal "foo" is a &'static str. The data is hardcoded into the executable and loaded into memory when the program runs. 
    > **Inside a heap allocated String:** String dereferences to a &str view of the String's data.
    > **On the stack**: e.g. the following creates a stack-allocated byte array, and then gets a view of that data as a &str: 
```rust
use std::str;

let x: [u8; 3] = [b'a', b'b', b'c'];
let stack_str: &str = str::from_utf8(&x).unwrap();
```
```rust
use std::str;

let my_string = String::from_string("hello, world!");   // String type
let my_slice = &my_string[0..5];                        // string literal type
```

> `String` keeps the buffer and is very practical to use. &str is lightweight and should be used to "look" into strings. You can search, split, parse, and even replace chunks without needing to allocate new memory.

> `&str` can look inside of a String as it can point to some string literal. The following code needs to copy the literal string into the String managed memory: `let a: String = "hello rust".into();`

More information can be found in this [article](https://dev.to/dsysd_dev/string-vs-str-in-rust-understanding-the-fundamental-differences-for-efficient-programming-4og8#:~:text=In%20summary%2C%20%22String%22%20is,and%20requirements%20of%20your%20code.)

**Structure in Rust**
> A struct is like an objects data attributes
> ```rust
> Struct User {
>    active: bool,
>    username: String,
>    email: String,
>    sign_in_count: u64,
> }
> ```

**Struct Tuple**
> Tuple structs are useful when you want to give the whole tuple a name and make the tuple different type from other tuples
> This makes the naming of each struct fields as in regular struct would be verbose or redundant
> Tuple struct instances are similar to tuples in that you can destructure them into their individual pieces and we can use a `.` operator followed by index to access an individual value

**Unit-Like Structs**
> This type of structs don't have any fields
> It can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

**Ownership of Struct Data**
> In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.
> It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes

**println!**
> The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as Display: output intended for direct end user consumption. The primitive types we’ve seen so far implement **Display Trait** by default because there’s only one way you’d want to show a 1 or any other primitive type to a user.
> But with structs, the way println! should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown?
> The println! macro call will now look like println!("rect1 is {:?}", rect1);. Putting the specifier :? inside the curly brackets tells println!
we want to use an output format called **Debug Trait**. The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code
> Another way to print out a value using the Debug format is to use the `dbg!` macro, which takes ownership of an expression (as opposed to println!, which takes a reference), prints the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.
> Calling the `dbg!` macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout). 

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
12. [Day 11: Handling Heap](https://inpyjama.com/day-11-handling-strings/)
13. [Day 12: Strings](https://inpyjama.com/day-12-strings/)

# Other important links
1. [UTF8 Decoder](https://www.browserling.com/tools/utf8-decode)
2. [Full Emoji List](https://www.unicode.org/emoji/charts/full-emoji-list.html)
3. [Emoji Unicode Tables](https://apps.timwhitlock.info/emoji/tables/unicode)
4. [Writing an OS in Rust](https://os.phil-opp.com/)
5. https://fasterthanli.me/
6. [Blog_OS Github Repository](https://github.com/phil-opp/blog_os) - Writing an OS in Rust