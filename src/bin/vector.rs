#[derive(Debug)]
enum SpreadsheetCell<'a> {
    Int(i32),
    Float(f64),
    Text(&'a str),
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    // let first = v.get(0).unwrap();

    v.push(7);
    println!("{:?}", v);

    // println!("first is {first:?}");
    // println!("v is {v:?}");

    for i in &mut v {
        *i += 10;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(2.0),
        SpreadsheetCell::Text("my text"),
    ];

    println!("{row:#?}");

    let mut s = String::from("hello");
    {
        let s_slice = &mut s[..]; // 获得 String 的可变引用
        s_slice.make_ascii_uppercase(); // 修改字符串的内容
    }
    println!("{}", s); // 打印 "HELLO"
    
    let ch = "你是谁";
    let s = ch.chars().nth(0).unwrap().to_string();
    println!("s is {s}");
    println!("ch is {ch}");
    // println!("{}", s);
    // for c in ch.chars() {
    //     println!("{}", c);
    // }
}
