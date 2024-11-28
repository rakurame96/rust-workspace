enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(5, 10, 15);
    let msg1 = Message::Quit;
    let msg2 = Message::Write(String::from("hello, rust"));
    let msg3 = Message::Move { x: 5, y: 10 };

    let v = vec![msg, msg1, msg2, msg3];

    // storing all values in the Vector and iterating over the vector to simulate all possible cases
    for v_item in v {
        match v_item {
            Message::ChangeColor(r, g, b) => println!(
                "Change color to red {r}, green {g}, and blue {b}"
            ),

            Message::Move { x, y } => println!(
                "Move in the x dir {x}, in the y dir {y}"
            ),

            Message::Quit => println!(
                "The Quit variant has no data to destructure"
            ),

            Message::Write(text) => println!(
                "Text message: {text}"
            ),
        }
    }
}
