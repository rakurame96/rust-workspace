fn main() {
    let s = String::from("hello");

    takes_ownership(s);
    // takes_ownership(s.clone());

    // println!("{}", s);      // throws error as s's value moves into the function
                            // and so no longer valid here
    // to make the above line work, call takes_ownership(s.clone())
    
    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer:i32) {
    println!("{some_integer}");
}