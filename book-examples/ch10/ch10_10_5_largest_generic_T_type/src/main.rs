// to fix uncomment the below line
// use std::cmp::PartialOrd;

// fn largest<T: PartialOrd>(list: &[T]) -> &T {
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 100, 65, 25];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['r', 'a', 'k', 'u', 'r', 'a', 'm'];
    let result = largest(&char_list);
    println!("The largest char is {result}");
}
