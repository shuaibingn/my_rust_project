use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut rng = thread_rng();

    let random_num = rng.gen_range(0..10);
    println!("random number: {}", random_num);

    loop {
        let mut input_number = String::new();
        io::stdin().read_line(&mut input_number).unwrap();

        let input_number = match input_number.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input_number.cmp(&random_num) {
            Ordering::Less => {
                println!("less");
            }
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => {
                println!("greater");
            }
        }
    }
}
