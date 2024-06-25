// below line will not work. As our trait name is ch10-10-13-trait-example not aggregator
// in the book example, aggregator is the trait name
// use aggregator::{Summary, Tweet};
use ch10_10_13_trait_example::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse books"),
        content: String::from("of course, as you probably already know, people"),
        reply: false, 
        retweet: false,
    };

    // let summary = Summary {

    // }

    println!("1 new tweet: {}", tweet.summarize());
}
