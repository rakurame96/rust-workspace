fn main() {
    let s = String::from("hello");      // s comes into the scope

    takes_ownership(s);             // s's value moves into the function...
                                                // ... and so no longer valid here
    // takes_ownership(s.clone());

    // println!("{}", s);      // throws error as s's value moves into the function
                            // and so no longer valid here
    // to make the above line work, call takes_ownership(s.clone())
    
    let x = 5;                              // x comes into scope

    makes_copy(x);                  // x would move into the function 
                                                 // but i32 is a copy, so it is okay to still
                                                 // use x afterward
}   // here x goes out of scope, then s. However, because s's value was moved,
    // nothing special happens here

fn takes_ownership(some_string: String) {       // some_string comes into the scope
    println!("{some_string}");
}   // here, some_string goes out of the scope and 'drop' is called. the backing
    // memory is freed

fn makes_copy(some_integer:i32) {               // some_integer comes into the scope
    println!("{some_integer}");
}   // here, some_integer goes out of the scope. nothing special happens here