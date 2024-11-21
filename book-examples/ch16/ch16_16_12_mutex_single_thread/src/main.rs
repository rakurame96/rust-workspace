use std::sync::Mutex;

fn main() {
    // new(10) = creates a variable m with value 10 or whatever value is given inside new()
    let m = Mutex::new(10);
    {
        println!("before acquiig lock m = {:?}", m);
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("after acquiring lock m = {:?}", m);
}
