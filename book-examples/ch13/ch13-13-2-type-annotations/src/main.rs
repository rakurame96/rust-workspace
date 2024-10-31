use std::thread;
use std::time::Duration;

/// 
/// Closure and function prototype comparision
/// function -> fn add_one_v1 (i1: u32) -> u32 { x + 1 }
/// closure v2 -> let add_one_v2 = |x: u32| -> u32 { x + 1 }
/// closure v3 -> let add_one_v3 = |x| { x + 1 };
/// closure v4 -> let add_one_v4 = |x| x + 1;
/// 

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
     };
    
    println!("Before closure execution");
    expensive_closure(10);
    println!("After closure execution");
}