use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // uncomment and build for error (value borrowed here after move)
        // println!("val is {:?}", val);     
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
