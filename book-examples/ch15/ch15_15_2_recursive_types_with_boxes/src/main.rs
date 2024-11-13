// enum List {
//     Cons(i32, List),
//     Nil,
// }

#[derive(Debug)]
#[allow(unused)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}


use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(
        1, 
        Box::new(Cons(
            2, 
            Box::new(Cons(
                3, Box::new(Nil)
            ))
        ))
    );

    println!("List: {:?}", list);
}
