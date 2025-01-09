fn main() {
    let s = ['R', 'U', 'N', 'o', 'O', 'b'];

    for a in s.iter() {
        print!("{}", a);
    }

    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'o' {
            break i;
        }
        println!("ch is {ch}");

        i += 1;
    };

    println!("location is {location}");
}
