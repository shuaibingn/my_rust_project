use std::collections::HashMap;

fn pig_latin(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut chars = s.chars();

    let first_chat = chars.next().unwrap();
    if vowels.contains(&first_chat) {
        return format!("{s}-hay");
    }
    format!("{}-{first_chat}ay", chars.as_str())
}

fn main() {
    let mut array = [6, 6, 2, 3, 4, 1, 1];

    for _ in 0..array.len() - 1 {
        for j in 1..array.len() {
            if array[j - 1] > array[j] {
                (array[j - 1], array[j]) = (array[j], array[j - 1]);
            }
        }
    }

    let mut num_map = HashMap::new();
    let mut max_count = 0;
    for num in array.iter() {
        let count = num_map.entry(num).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
        }
    }

    for (k, v) in num_map.iter() {
        if *v == max_count {
            println!("most num {k}");
        }
    }

    let middle_number = array[array.len() / 2];
    println!("middle num {middle_number:?}");

    let result = pig_latin("apple");
    println!("{result}");
}
