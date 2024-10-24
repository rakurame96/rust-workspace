fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("a : {:?}", a);

    let slice = &a[1..3];
    println!("slice : {:?}", slice);

    // println!("assertion : {:?}", assert_eq!(slice, &[2, 3]));
}
