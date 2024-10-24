fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // prints with enum
    println!("value of six : {:?}", six);

    // prints only the value upon unwrapping
    println!("value of six : {:?}", six.unwrap());
    
    println!("value of none : {:?}", none);
    
}
