use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    #[allow(unused_doc_comments)]
    // by moving the join() before the main for loop, spawned thread will execute 1st and later main for loop will get executed
    // output
    /**
     * hi number 1 from the spawned thread!
     * hi number 2 from the spawned thread!
     * hi number 3 from the spawned thread!
     * hi number 4 from the spawned thread!
     * hi number 5 from the spawned thread!
     * hi number 6 from the spawned thread!
     * hi number 7 from the spawned thread!
     * hi number 8 from the spawned thread!
     * hi number 9 from the spawned thread!
     * hi number 1 from the main thread!
     * hi number 2 from the main thread!
     * hi number 3 from the main thread!
     * hi number 4 from the main thread!
     */
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // having join() at end ensures that both main for loop and spawned thread runs concurrently. 
    // Once the main for loop execution ends, it will wait for the spawned thread to complete
    /*
        hi number 1 from the main thread!
        hi number 1 from the spawned thread!
        hi number 2 from the main thread!
        hi number 2 from the spawned thread!
        hi number 3 from the main thread!
        hi number 3 from the spawned thread!
        hi number 4 from the main thread!
        hi number 4 from the spawned thread!
        hi number 5 from the spawned thread!
        hi number 6 from the spawned thread!
        hi number 7 from the spawned thread!
        hi number 8 from the spawned thread!
        hi number 9 from the spawned thread!
     */
    handle.join().unwrap();
}
