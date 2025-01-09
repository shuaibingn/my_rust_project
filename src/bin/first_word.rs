fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..s.len()]
}
