// fn largest_i32(list: &Vec<i32>) -> &i32 {
//     let mut largest = &list[0];
//     for i in 1..list.len() {
//         if list[i] > *largest {
//             largest = &list[i];
//         }
//     }
//     largest
// }

fn largest<T>(list: &Vec<T>) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];
    for i in 1..list.len() {
        if list[i] > *largest {
            largest = &list[i];
        }
    }
    largest
}

struct Point<I, F> {
    x: I,
    y: F,
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<I, F> Point<I, F> {
    fn x(&self) -> &I {
        &self.x
    }

    fn y(&self) -> &F {
        &self.y
    }
}

fn main() {
    let list = vec!['1', '2', 'd'];
    let largest = largest(&list);
    println!("The largest value is {}", largest);

    let integer = Point { x: 5, y: 10.0 };
    let float = Point { x: 1.0, y: 4 };
}
