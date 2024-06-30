use std::fmt;

pub trait Summary {
    fn summarize_author(&self) -> String;
    // only function signature 
    // implementation could be different depending on who is using that trait
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl fmt::Display for dyn Summary{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summarize())
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

#[derive(Debug)]
pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool, 
}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "username: {}, content: {}, reply: {}, retweet: {}", self.username, self.content, self.reply, self.retweet)
    }
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
