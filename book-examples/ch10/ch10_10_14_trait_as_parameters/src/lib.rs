pub trait Summary {
    fn summarize_author(&self) -> String;
    // only function signature 
    // implementation could be different depending on who is using that trait
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// regular implementation
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax - same functionality as above
pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/*
// this method can be used when both item1 & item2 points to different types
pub fn notify(item1: &impl Summary, item2: &impl Summary){

}

// trait bound syntax method can be used when both item1 & item2 points to same type
pub fn notify<T: Summary> (item1: &T, item2: &T){

}

// specify multiple trait bounds with + syntax
pub fn notify(item: &(impl Summary + Display)) {

}

pub fn notify<T: Summary + Display>(item: &T) {

}

// Clearer Trait Bounds with where Clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {

}

// other method
fn some_function<T, U>(t: &T, u: &U) -> i32 
where
T: Display + Clone,
U: Clone + Debug,
{

}

*/

pub struct NewsArticle {
    pub headline: String, 
    pub location: String,
    pub author: String, 
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool, 
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
