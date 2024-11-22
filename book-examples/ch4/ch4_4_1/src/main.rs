fn main() {
    
    {                       // s is not valid here since it's not yet declared
        let s = "hello";    // s is valid from this point forward

        println!("value of s : {s}");
    }                       // this scope is now over, and s is no longer valids
    // println!("value of s : {s}");

    // using string keyword -> this data will be stored in the heap instead of stack
    let s = String::from("hello");

    println!("value of s in outer scope: {s}");

    let mut s = String::from("hello");

    s.push_str(", world!");

    assert_eq!("hello, world!", s);

    println!("value of s after adding further characters: {s}");
}
