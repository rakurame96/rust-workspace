mod front_of_house;
    

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // checks the crate root file for deliver_order() function implementation
    }
    fn cook_order() {}
}

fn deliver_order() {}

// bring the crate into scope
// use crate::front_of_house::hosting;
// use front_of_house::hosting;      // still works though

// bring the crate into scope
use crate::front_of_house::hosting;
use crate::back_of_house::{Breakfast, Appetizer};   // breakfast is struct & Appetizer is an Enum
    
pub fn eat_at_restaurant() {
    // crate::front_of_house::hosting::add_to_waitlist();      // instead of using complete path, we can bring the hosting into scope by use keyword
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast.
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not
    // allowed to see or modify the seasonal fruit that comes
    // with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    // enums
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}
