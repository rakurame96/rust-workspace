pub trait Summary {
    // fn summarize_author(&self) -> String;
    // only function signature 
    // implementation could be different depending on who is using that trait
    fn summarize(&self) -> String {
        format!("(Read more from...)")
    }
}

pub struct NewsArticle {
    pub headline: String, 
    pub location: String,
    pub author: String, 
    pub content: String,
}

impl Summary for NewsArticle {

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

    // fn summarize_author(&self) -> String {
    //     format!("@{}", self.username)
    // }
}