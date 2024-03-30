fn main() {
    // while Loop through a array
    let a = [10, 20, 30, 40, 50];

    // ------------------------- //
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is : {}", a[index]);

    //     index += 1;
    // }

    // println!("printed all elements in an array");
    // ------------------------- //
    
    // same operation can be achieved using for loop

    for element in a {
        println!("the value is : {element}");
    }

}
