fn main() {
    let s1 = gives_overship();
    println!("s1 : {}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    println!("s3 : {}", s3);
}

fn gives_overship() -> String {
    let some_string = String::from("yours");

    // some_string         // return value to the caller
    return some_string;
}

fn takes_and_gives_back(mut a_string: String) -> String {
    a_string.push_str(" rustaceans!!!");
    
    a_string
}
