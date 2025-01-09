use std::fs::File;
use std::path::Path;

fn create_file(file_name: &str) -> Result<File, std::io::Error> {
    if Path::new(file_name).exists() {
        return File::open(file_name);
    }
    File::create(file_name)
}

fn main() {
    // let mut greeting_file_result = File::open("hello.txt");
    //
    // // let greeting_file = match greeting_file_result {
    // //     Ok(file) => file,
    // //     Err(err) => {
    // //         panic!("something went error: {err:?}");
    // //     }
    // // };
    //
    // if let Ok(file) = &mut greeting_file_result {
    //     let mut contents = String::new();
    //     file.read_to_string(&mut contents).unwrap();
    //     println!("{}", contents);
    //     return;
    // }
    //
    // let mut file = File::create("hello.txt").unwrap();
    // file.write_all(b"123").unwrap();

    _ = create_file("recovery_error.txt");
}
