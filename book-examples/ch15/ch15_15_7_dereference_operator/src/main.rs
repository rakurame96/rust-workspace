fn main() {
    let x = 5;
    let y = Box::new(x);
    
    println!("y = {:p}", y);
    println!("*y = {}", *y);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
