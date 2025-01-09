fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.0;
    let quotient = 56 / 32;
    let remainder = 43 % 5;
    println!("Sum: {sum}, difference: {difference}, quotient: {quotient}, remainder: {remainder}");

    let tup: (i32, f64, u8) = (500, 23.2, 1);
    println!("tup 1 {}", tup.1);

    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z: {z}");

    let a = [1, 2, 3, 4, 5];
    println!("a = {a:?}, len = {len}", len = a.len());

    let b = ["January", "February", "March"];
    println!("b = {b:?}");

    let mut c = [1; 10];
    println!("c = {c:?}");

    c[c.len()-1] = 2;
    println!("c = {c:?}");
}
