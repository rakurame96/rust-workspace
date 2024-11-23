fn main() {
    let mut v = vec![1, 2, 3, 4, 5];    // mutable vector
    // let first = &v[0];                      // immutable borrowing of vector v
    v.push(6);                                    // mutable borrowing
    // println!("The first element is: {first}");
    println!("print vector v : {:?}", v);

    // below code won't crash, as we add the new element into the vector v, then printing them. 
    // in-between if we add any code which has immutable reference, then compilation won't happen
    let first = &v[0];
    println!("The first element is: {first}");

    // explanation
    /*
    The code in Listing 8-6 might look like it should work: 
    why should a reference to the first element care about changes at the end of the vector? 
    This error is due to the way vectors work: because vectors put the values next to each other in memory, 
    adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, 
    if there isn’t enough room to put all the elements next to each other where the vector is currently stored. 
    In that case, the reference to the first element would be pointing to deallocated memory. 
    The borrowing rules prevent programs from ending up in that situation
     */
}
