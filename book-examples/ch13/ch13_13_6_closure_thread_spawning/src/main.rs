use std::thread;

fn main() {
    let list =  vec![1, 2, 3, 5];
    println!("Before defining the closure: {:?}", list);

    thread::spawn(move || {
        println!("From Thread: {:?}", list)
    }).join().unwrap();
}
