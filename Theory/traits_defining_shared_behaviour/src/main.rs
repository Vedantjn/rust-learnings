// Define a struct for NewsArticle
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}  

// Define a struct for Tweet
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

fn main() {
    println!("Hello, world!");
}

// Function to handle tweets in news aggregator
fn news_aggregator_tweet(tweet: Tweet) {
    println!("There is a new news in town!");
    println!("The news is that {} and it is published by {}", tweet.content, tweet.username);
}

// Function to handle news articles in news aggregator
// Note: This function has an error in its parameter type, it should be NewsArticle instead of Tweet
fn news_aggregator_news(news: NewsArticle) {
    println!("There is a new news in town!");
    println!("The news is that {} and it is published by {}", news.content, news.author);
}

// ---------------------------------------------------------------------------------------------------

// Define a trait for summarizing content
trait Summary {
    fn summarize(&self) -> String;
}

// Implement Summary trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Implement Summary trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    // Create a new Tweet instance
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // Create a new NewsArticle instance
    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    // Call summarize method on tweet
    tweet.summarize();
    // Pass tweet and news to news_aggregator function
    news_aggregator(tweet);
    news_aggregator(news);
}

// Function that takes any type implementing Summary trait
fn news_aggregator(source: impl Summary) {
    println!("There is a new news in town!");
    println!("{}", source.summarize());
}

// ---------------------------------------------------------------------------------------------------

// Define Summary trait with default implementation for summarize method
trait Summary {
    fn get_author(&self) -> &str;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.get_author())
    }
}

// Define another trait
trait MyTrait {
    fn demo(&self) -> String;
}

// Implement Summary trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn get_author(&self) -> &str {
        &self.username
    }
}

// Implement Summary trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn get_author(&self) -> &str {
        &self.author
    }
}

// Implement MyTrait for Tweet
impl MyTrait for Tweet {
    fn demo(&self) -> String {
        format!("{}", self.content)
    }
}  

fn main() {
    // Create Tweet and NewsArticle instances
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    // Call methods and functions
    tweet.summarize();
    news_aggregator(&tweet);
    news_aggregator(&news);
    mixup_news(&tweet);
}

// Function that takes a reference to any type implementing Summary trait
fn news_aggregator(source: &impl Summary) {
    println!("There is a new news in town!");
    println!("{}", source.summarize());
}

// Commented out alternative implementations of get_news and mixup_news functions
// fn get_news<T: Summary>(source: T) {
//     println!("There is a new news in town!");
//     println!("{}", source.summarize());
// }

// fn mixup_news<T: Summary>(primary_source: &T, secondary_source: &T) {
//     println!("There is a new news in town!");
//     println!("{}", primary_source.summarize());
//     println!("{}", secondary_source.summarize());
// }

// fn mixup_news(primary_source: &impl Summary, secondary_source: &impl Summary) {
//     println!("There is a new news in town!");
//     println!("{}", primary_source.summarize());
//     println!("{}", secondary_source.summarize());
// }

// Function that takes a reference to any type implementing both Summary and MyTrait traits
fn mixup_news(source: &(impl Summary + MyTrait)) {
    println!("There is a new news in town!");
    println!("{}", source.summarize());
    println!("{}", source.demo());
}