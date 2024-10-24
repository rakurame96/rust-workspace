#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
//  --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

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
        Coin::Quarter(state) => {            
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let value = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Value of Quarter : {}", value);

    let value = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Value of Quarter : {}", value);

    let value = value_in_cents(Coin::Dime);
    println!("Value of Quarter : {}", value);

    let value = value_in_cents(Coin::Nickel);
    println!("Value of Quarter : {}", value);

    let value = value_in_cents(Coin::Penny);
    println!("Value of Quarter : {}", value);
}
