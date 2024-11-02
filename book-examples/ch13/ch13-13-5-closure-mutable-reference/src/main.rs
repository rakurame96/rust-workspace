#[allow(unused_doc_comments)]
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || {
        println!("from closure before adding: {:?}", list);
        list.push(7);
        println!("from closure after adding: {:?}", list);
    };

    ///
    /// we can beautify the closure like this also
    /// let only_borrows = || 
    /// {
    ///     println!("from closure: {:?}", list);
    /// }

    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}
