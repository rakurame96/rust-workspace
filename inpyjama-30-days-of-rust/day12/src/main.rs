fn main() {
    let mut message = String::from("Rust🦀"); 
    println!("First elemnt of message is {}", message); 
                                                  //  ^ `String` cannot be indexed by `{integer}`
    println!("Length of the string is: {}", message.len());

    // using bytes method, we can able to read individual byte as well
    // this includes 'rust' symbol as 4 byte data
    println!("message.bytes().nth(4) : 0x{:X} ", message.bytes().nth(4).unwrap());
    println!("message.bytes().nth(5) : 0x{:X} ", message.bytes().nth(5).unwrap());
    println!("message.bytes().nth(6) : 0x{:X} ", message.bytes().nth(6).unwrap());
    println!("message.bytes().nth(7) : 0x{:X} ", message.bytes().nth(7).unwrap());
    // println!("message.bytes().nth(8) : {} ", message.bytes().nth(8).unwrap());

    // using len_utf8 method
    println!("char [0]: 0x{:X}, size : {} ", message.bytes().nth(0).unwrap(), message.chars().nth(0).unwrap().len_utf8());
    println!("char [1]: 0x{:X}, size : {} ", message.bytes().nth(1).unwrap(), message.chars().nth(1).unwrap().len_utf8());
    println!("char [2]: 0x{:X}, size : {} ", message.bytes().nth(2).unwrap(), message.chars().nth(2).unwrap().len_utf8());
    println!("char [3]: 0x{:X}, size : {} ", message.bytes().nth(3).unwrap(), message.chars().nth(3).unwrap().len_utf8());
    println!("char [4]: 0x{:x}, size : {} ", message.bytes().nth(4).unwrap()., message.chars().nth(4).unwrap().len_utf8());

    println!("message.bytes().nth(5) : 0x{:X} ", message.bytes().nth(5).unwrap());
    println!("message.bytes().nth(6) : 0x{:X} ", message.bytes().nth(6).unwrap());
    println!("message.bytes().nth(7) : 0x{:X} ", message.bytes().nth(7).unwrap());

    // using char method because chars reads the string like below
    // 'r', 'u', 's', 't', & '🦀'
    // that's why, we could not able to read individual characters of 'rust' symbol
    println!("message.chars().nth(4) : {} ", message.chars().nth(4).unwrap().);
    // println!("message.chars().nth(5) : {} ", message.chars().nth(5).unwrap());   // uncomment to see rust getting 'panicked'
    // println!("message.chars().nth(6) : {} ", message.chars().nth(6).unwrap());
    // println!("message.chars().nth(7) : {} ", message.chars().nth(7).unwrap());

    // size of above string is not 1 byte per character
    // unicode and UTF-8 varies in size
    // Rust needs 4 bytes for 🦀, but R can be represented using a single byte.
    message.push_str(" rustaceans!!");
    println!("First elemnt of message is {}", message); 

    println!("Length of the string is: {}", message.len());

    println!("message.bytes().nth(5) : {} ", message.bytes().nth(5).unwrap());
    println!("message.bytes().nth(6) : {} ", message.bytes().nth(6).unwrap());
    println!("message.bytes().nth(7) : {} ", message.bytes().nth(7).unwrap());
    println!("message.bytes().nth(8) : {} ", message.bytes().nth(8).unwrap());

}