fn main() {
    let a = 123;

    // a = "abc";
    // a = 4.45
    // a = 456
    println!("a is {a}");

    let mut b = 456;
    println!("b is {b}");

    // b = "abc";
    // b = 4.45
    b = 789;
    println!("b is {b}");

    let x = 42;
    let y = 3.14;
    let is_true = true;
    let letter = 'A';
    println!("x is {x}, y is {y}, {is_true}, letter is {letter}");

    let m: i32 = 12;
    let n: i32 = 45;
    let result = add(m, n);
    println!("m + n = {result}");

    let number = 7;
    if number < 5 {
        println!("5");
    } else {
        println!("greater than 5");
    }

    let mut counter = 0;
    loop {
        counter += 1;
        println!("loop counter is {counter}");
        if counter == 10 {
            break;
        }
    }

    let mut number = 3;
    while number != 0 {
        println!("number is {number}");
        number -= 1;
    }

    for i in 0..=4 {
        println!("for range {i}");
    }

    // 所有权
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 is {s2}");

    // 借用
    let s1 = String::from("hello");
    let mut s2 = &s1;
    println!("s1 is {s1}, s2 is {s2}");

    let s3 = String::from("world");
    s2 = &s3;
    println!("s2 is {s2}, s3 is {s3}");

    let user1 = User {
        username: String::from("some username"),
        email: String::from("someone@example.com"),
        sign_in_count: 64,
        active: true,
    };

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

enum IpAddrKind {
    V4,
    V6,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Divide by zero"));
    }
    Ok(a / b)
}

fn get_element(index: usize, vec: &Vec<i32>) -> Option<i32> {
    if index < vec.len() {
        return Some(vec[index]);
    }
    None
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         return x;
//     }
//     y
// }

fn add(a: i32, b: i32) -> i32 {
    a + b
}
