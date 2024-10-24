fn main() {
    let s1 = gives_overship();      // gives_ownership moves its return value into s1
    println!("s1 : {}", s1);

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 moved into takes_and_gives_back, which also move its
                                                         // return value into s3
    println!("s3 : {}", s3);
}   // s3 goes out of scope and is dropped. s2 was moved, so nothing happens
    // s1 goes out of scope and is dropped

fn gives_overship() -> String {             // gives_overship will move its return value into the function that calls it.
    let some_string = String::from("yours");    // some_string comes into scope

     // some_string         // return value to the caller
    return some_string;     // some_string is returned and
                            // moves out to the calling
                            // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(mut a_string: String) -> String {   // a_string comes into scope with mutability
    a_string.push_str(" rustaceans!!!");
    
    a_string    // a_string is returned and moves out to the calling function with additional character attached to it because of mutability
}
