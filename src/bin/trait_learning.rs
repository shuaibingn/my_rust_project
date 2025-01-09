use std::fmt::Display;
use std::ops::Add;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Tweet {
    fn add<T>(a: T, b: T) -> T
    where
        T: Add<Output = T>,
    {
        a + b
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.content)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

impl<T> Summary for Vec<T>
where
    T: Summary,
{
    fn summarize_author(&self) -> String {
        "".to_string()
    }

    fn summarize(&self) -> String {
        let summaries: Vec<String> = self.iter().map(|item| item.summarize()).collect();
        summaries.join(", ")
    }
}

// fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Breaking news! {}", item.summarize());
}

pub fn returns_summarize() -> impl Summary {
    Tweet {
        username: "Horse_ebook".to_string(),
        content: "of course! as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: "Horse_ebook".to_string(),
        content: "of course! as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    println!("{tweet}");

    let tweet1 = returns_summarize();
    println!("{}", tweet1.summarize_author());
}
