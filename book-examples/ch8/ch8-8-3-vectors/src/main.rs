fn main() {
    let mut v1: Vec<i32> = Vec::new();      // empty vector

    let mut v2 = vec![1, 2, 3];   // initialized vector using vec![] macro  

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

	// let mut v2 = Vec::new();
	v2.push(4);
	println!("Vector v2 : {:?}", v2);
}
