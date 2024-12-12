#[allow(dead_code)]
trait Pilot {
    fn fly(&self);
}

#[allow(dead_code)]
trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!!!");
    }
}

// default implementation
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;

    // the compiler defaults to calling the method that is directly implemented on the type
    person.fly();
}
