// Traits allow to define methods that are shared across different types
// So traits is basically interface but allow default or concrete implementation
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String 
}

impl Summary for NewsArticle {
    // Overriden implementation
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.content)
    }

    fn introduce_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    // Overriden implementation
    fn summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }

    fn introduce_author(&self) -> String {
        format!("Author: {}", self.username)
    }
}

pub trait Summary {
    // Default implementation
    fn summarize(&self) -> String {
        String::from("Read more...")
    }

    fn introduce_author(&self) -> String;
}

fn main() {
    let tweet1 = Tweet {
        username: String::from("John"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false
    };

    println!("{}", tweet1.summarize());
    notify(&tweet1);
    notify_people(&tweet1);

    let tweet2 = return_an_summarizable();
    notify(&tweet2);
}

// trait as bond or type passed in function 
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_people<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());    
}

pub fn notify_me<T>(t: &T) where T: Summary {
    println!("Breaking news! {}", t.summarize());
}

fn return_an_summarizable() -> impl Summary {
    Tweet {
        username: String::from("John"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false
    }
}