fn main() {
    let x = 1;

    match x {
        // below line matches for either 1/2/3 from the 'x' if none is matched then it will check for the next possible match instead
        1 | 2 | 3 => println!("one or two"),
        4 => println!("four"),
        _ => println!("anything"),
    }
}
