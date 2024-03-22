In rust, once the value assigned to the variable, the value cannot be changed again. If we try to modify the value, we end up getting the below error.
* Oops, the code doesn't compile! The compiler complains: "cannot assign twice to immutable variable x".
* By default, all variables in Rust are immutable (read-only). 

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

> **_NOTE: Why unsafe keyword is necessary in the above situation_**
Looking at the error, it complains that using mutable statics is unsafe and needs an unsafe block. It also mentions the potential for data races if increment_counter is called from multiple threads. Rust isn't being picky – it's absolutely right! Why? Because multiple threads could potentially call increment_counter simultaneously, leading to a data race – a situation where multiple threads access the same memory location without proper synchronization. This can corrupt the value of COUNTER.

This is how Rust ensures memory safety – by refusing to compile code that could lead to trouble. However, there might be times when you absolutely need a mutable static. The compiler leaves us a hint: mutable statics require an unsafe block. Think of the unsafe keyword as a backdoor for special cases where you need to bypass Rust's usual safety checks.

##### Variable Shadowing
You can redeclare a variable with the same name using the let keyword. This is called variable shadowing in Rust. The new declaration shadows the previous one. But hold on, what happens to the old variable? The key here is that shadowing creates an entirely new variable in a different memory space. The original isn't modified or replaced. The shadowing re-binds the variable name to a new memory location. The original memory location still exists, and the value it holds is technically still there. The important point is that you can no longer access that old value using the same variable name.

### Reference
* [Day 6: Variables Continued](https://inpyjama.com/day6-variables-contd/)