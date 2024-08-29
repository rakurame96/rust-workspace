#![allow(unused)]

use futures::{join, pin_mut, select, FutureExt};

fn main() {
    let num1 = num1().fuse();
    let num2 = num2().fuse();
    let num3 = num3().fuse();

    pin_mut!(num1, num2, num3);
    
    let result = smol::block_on(
        async {
            // join!(num1, num2, num3)
            loop {       
                select! {
                    x = num1 => println!("num1 is completed {}", x),
                    y = num2 => println!("num2 is completed {}", y),
                    z = num3 => println!("num3 is completed {}", z),
                    complete => {
                        println!("all futures have finished polling. Breaking out of loop!!");
                        break;
                    }

                }
            }
        }
    );
    println!("The final value {:?}", result);
}

async fn num1() -> u8 {
    println!("Running num1");
    10
}

async fn num2() -> u8 {
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Running num2");
    50
}

async fn num3() -> u8 {
    std::thread::sleep(std::time::Duration::from_secs(6));
    println!("Running num3");
    100
}