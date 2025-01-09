use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn suqare(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 12,
        height: 12,
    };
    let rect1_area = area(&rect1);
    println!("The area of rect1 is {}", rect1_area);

    let area_from_method = (&rect1).area();
    println!("{}", area_from_method);
    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle {
        width: 11,
        height: 10,
    };
    let hold_result = rect1.can_hold(&rect2);
    println!("{}", hold_result);

    let square = Rectangle::suqare(10);
    let square_area = square.area();
    println!("{}", square_area);

    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let some_number = Some(5);
    let x: usize = 5;
    let result = some_number.unwrap_or_default() + x;
    println!("{}", result);
}
