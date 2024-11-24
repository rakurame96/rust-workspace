// below line will not work. As our trait name is ch10-10-13-trait-example not aggregator
// in the book example, aggregator is the trait name
// use aggregator::{Summary, Tweet, NewsArticle};
use ch10_10_14_trait_default_impl::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse books"),
        content: String::from("of course, as you probably already know, people"),
        reply: false, 
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Champioship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());
}
