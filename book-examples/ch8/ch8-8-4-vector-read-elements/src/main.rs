fn main() {
    let mut v1: Vec<i32> = Vec::new();      // empty vector

    let v2 = vec![1, 2, 3, 4, 5];   // initialized vector using vec![] macro  

	// initialing vector using push() method
	v1.push(5);
	v1.push(6);
	v1.push(7);
	v1.push(8);

	// dbg!("Vector v1: ", v1);
	// dbg!("Vector v2: ", v2);

	println!("Size of Vector v1: {} ", v1.len());
	println!("Size of Vector v2: {}", v2.len());

	println!("Vector v1 : {:?}", v1);
	println!("Vector v2 : {:?}", v2);

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("there is no thrid element"),
    }

    // let does_not_exist = &v2[100];      // uncomment and compile - program will panic
    let does_not_exist = v2.get(100);

    match does_not_exist {
        Some(does_not_exist) => println!("the element does_not_exist is {does_not_exist}"),
        None => println!("there is no does_not_exist element"),
    }
}
