// use std::mem::size_of;
use std::mem::size_of;

fn main() {
    let myvar:u8;                   // variable declaration
    let myvar1:u16 = 0x3FF;         // variable definition
    let myvar2 = 0x77FFF;           // without type declaration - still works 

    myvar = 10;
    println!("myvar 0x{:X}\n", myvar);
    println!("myvar1 0x{:X}\n", myvar1);
    println!("myvar2 0x{:X}\n", myvar2);
    println!("myvar2 {:?}\n", myvar2);

    println!("Data type sizes in Rust:");

    println!("bool:        {} bytes", size_of::<bool>());
    println!("char:        {} bytes", size_of::<char>());
    println!("i8:          {} bytes", size_of::<i8>());
    println!("i16:         {} bytes", size_of::<i16>());
    println!("i32:         {} bytes", size_of::<i32>());
    println!("i64:         {} bytes", size_of::<i64>());
    println!("i128:        {} bytes", size_of::<i128>());
    println!("u8:          {} bytes", size_of::<u8>());
    println!("u16:         {} bytes", size_of::<u16>());
    println!("u32:         {} bytes", size_of::<u32>());
    println!("u64:         {} bytes", size_of::<u64>());
    println!("u128:        {} bytes", size_of::<u128>());
    println!("f32:         {} bytes", size_of::<f32>());
    println!("f64:         {} bytes", size_of::<f64>());
    println!("usize:       {} bytes", size_of::<usize>());
    println!("isize:       {} bytes", size_of::<isize>());
}