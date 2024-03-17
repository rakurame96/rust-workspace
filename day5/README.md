To create a local variable in rust, use below syntax

#### Declaration: 
```rust
let my_variable: data_type;
```

### Definition: 
```rust 
let my_variable: data_type = value;
```

{:x} = displays hexadecimal notation in lowercase\
{:X} = displays hexadecimal notation in uppercase

> **_NOTE:_**
One key difference between strongly typed languages like C and loosely typed languages like Python lies in how you declare variables. In strongly typed languages like C, you must explicitly define the data type of each variable. Loosely typed languages, however, can infer the data type based on how the variable is used. Rust, while being strongly typed and compiled, adopts a slightly different approach.

> **_NOTE: Pointer Size_** 
The size of a pointer can vary depending on your computer's architecture. For instance:

* On a 32-bit system, pointers typically occupy 4 bytes
* On a 64-bit system, pointers often take up 8 bytes
Rust employs `usize` and `isize` to abstract away architecture-specific memory details, making your code more portable.

