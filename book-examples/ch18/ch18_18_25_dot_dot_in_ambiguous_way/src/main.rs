fn main() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // below code will not compile and throws error
        // using .. must be unambiguous. If it is unclear which values are intended for matching and which should be ignored, 
        // Rust will give us an error.
        // (.., second, ..) => {
        /* this code, second will match to the last value in the tuple not the 2nd value. this will ignore all tuple except for the last one */
        (.., second) => {   
            println!("Some numbers: {second}");
        },
    }
}
