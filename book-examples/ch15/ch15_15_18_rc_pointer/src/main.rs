// enum List {
//     Cons(i32, List),
//     Nil,
// }

#[derive(Debug)]
#[allow(unused)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a: {:?}", a);
    println!("count after creating a = {}", Rc::strong_count(&a));

    // Rc::clone() doesnot perform the deep copy so operation will be completed in short time
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("b: {:?}", b);
    println!("count after creating b = {}", Rc::strong_count(&a));
    println!("reference count of b after creating b = {}", Rc::strong_count(&b));
    {
        let c = Cons(7, Rc::clone(&a));
        println!("c: {:?}", c);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    {
        let d = Cons(4, Rc::clone(&b));
        println!("d: {:?}", d);
        println!("reference count of b after creating d = {}", Rc::strong_count(&b));
    }

    println!("reference count of b after d goes out of scope = {}", Rc::strong_count(&b));
}
