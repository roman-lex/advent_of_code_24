use std::env;
use std::fs;

fn main() {
    let file_path = String::from("input_day1_1.txt");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
