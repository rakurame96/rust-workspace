use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse books"),
        content: String::from("of course, as you probably already know, people"),
        reply: false, 
        retweet: false,
    };

    // let summary = Summary {

    // }

    println!("1 new tweet: {}", tweet.summarise());
}
