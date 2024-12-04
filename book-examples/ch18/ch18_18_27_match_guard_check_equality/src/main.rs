fn main() {
    let x = Some(05);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(n) if n == y => println!("matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end, x = {:?}, y = {}", x, y);
}
