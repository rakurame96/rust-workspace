trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

fn main() {
    // this line will print the out from impl dog "Spot" as a output
    // call the Dog::baby_name function, which calls the associated function defined on Dog directly
    println!("A baby dog is called a {}", Dog::baby_name());

    // output isn’t what we wanted. 
    // We want to call the baby_name function that is part of the Animal trait,
    // that we implemented on Dog so the code prints A baby dog is called a puppy

    // Attempting to call the baby_name function from the Animal trait, but Rust doesn’t know which implementation to use
    // println!("A baby dog is called a {}," Animal::baby_name());     // incorrect and error

    // Rust with a type annotation within the angle brackets, which indicates we want to call the baby_name method from the Animal trait
    // as implemented on Dog by saying that we want to treat the Dog type as an Animal for this function call
    println!(
        "A baby dog is called a {}", <Dog as Animal>::baby_name()
    );
}
