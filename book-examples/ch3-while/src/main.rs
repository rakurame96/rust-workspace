fn main() {

    // conditional loops with while
    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}");

    //     number -= 1;
    // }

    // println!("LIFTOFF");

    // while Loop through a array
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is : {}", a[index]);

        index += 1;
    }

    println!("printed all elements in an array");
}
