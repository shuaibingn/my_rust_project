use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}

fn longest_test<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let mut longest = x;
//     if y.len() > y.len() {
//         longest = y;
//     }
//     longest
// }

fn static_test() -> &'static str {
    "abcd"
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        return x;
    }
    y
}

fn main() {
    let x = "abc";
    let y = "def";
    let result = longest(x, y);
    println!("The longest string is {}", result);
    println!("x is {x}, y is {y}");

    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    {
        result = longest(&string1, &string2);
        println!("The longest string is {}", result);
    }
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i;
    {
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    let level = i.level();
    println!("level = {}", level);

    println!("The Important excerpt is {i:#?}");

    println!("{}", i.announce_and_return_part(&novel));

    let s = "I hava a static lifetime";
    println!("{}", s);

    let static_str = static_test();
    println!("{}", static_str);
}
