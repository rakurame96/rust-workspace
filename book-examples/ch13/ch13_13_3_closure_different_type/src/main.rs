fn main() {
    let example_closure = |x| x; 
    let s = example_closure(String::from("hello"));
    
    // uncomment and run the code. it will throw the error. As closure will imply the type automatically when it identifies the 1st call
    // accordingly, closure will infer the type and it can't be changed
    // error desription: expected because the closure was earlier called with an argument of type `String`
    // let n = example_closure(5);
    println!("s: {s}");
}
