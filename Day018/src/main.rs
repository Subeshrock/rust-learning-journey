pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
//     println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
// }
// pub fn notify2<T: Summary>(item1: &T, item2: &T) {
//     println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
// }

// pub fn notify3(item1: &impl Summary + Display, item2: &impl Summary) {
//     println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
// }
// pub fn notify3<T: Summary + Display>(item1: &T, item2: &T) {
//     println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
// }

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
//     println!("t: {}, u: {:?}", t, u);
//     42
// }

// the better way to write this is with a trait bound syntax
// fn some_function2<T, U>(t: T, u: U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     println!("t: {}, u: {:?}", t, u);
//     42
// }

fn main() {
    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("This is a tweet!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Author Name"),
        headline: String::from("Breaking News"),
        content: String::from("This is the content of the news article."),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    notify(&tweet);
    notify(&article);
}
