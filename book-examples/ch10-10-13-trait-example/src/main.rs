use std::{fmt::format, iter::Sum};

pub trait Summary {
    // only function signature 
    // implementation could be different depending on who is using that trait
    fn summarise(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String, 
    pub location: String,
    pub author: String, 
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool, 
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse books"),
        content: String::from("of course, as you probably already know, people"),
        reply: false, 
        retweet: false,
    }

    // let summary = Summary {

    // }

    println!("1 new tweet: {}", tweet.summarise());
}
