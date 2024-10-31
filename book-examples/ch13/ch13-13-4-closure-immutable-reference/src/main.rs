#[allow(unused_doc_comments)]
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("from closure: {:?}", list);

    ///
    /// we can beautify the closure like this also
    /// let only_borrows = || 
    /// {
    ///     println!("from closure: {:?}", list);
    /// }

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}
