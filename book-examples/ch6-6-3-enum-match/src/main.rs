enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// we can also have curly braces to do some computation once the pattern is matched
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }       // we can add ``,`` to separate the next pattern
        Coin::Nickel => {
            println!("Lucky Nickel");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let value = value_in_cents(Coin::Quarter);
    println!("Value of Quarter : {}", value);

    let value = value_in_cents(Coin::Dime);
    println!("Value of Quarter : {}", value);

    let value = value_in_cents(Coin::Nickel);
    println!("Value of Quarter : {}", value);

    let value = value_in_cents(Coin::Penny);
    println!("Value of Quarter : {}", value);
}
