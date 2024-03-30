fn main() {
    println!("Hello, world!");

    // -------------------------------- //
    // example - 1
    // below snippet will run continuously as long as user doesn't stop using `ctrl + c` input
    // loop {
    //     println!("Again");
    // }
    // -------------------------------- //

    // -------------------------------- //
    // example - 2 --> assigning the result of a loop directly to the another variable
    // loop returns the value back
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;
        
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");
    // -------------------------------- //

    // -------------------------------- //
    // example - 3 --> using loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up : loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    // -------------------------------- //
}
