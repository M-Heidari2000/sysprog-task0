use std::io::{self, Read};

fn main() {
    let mut file_contents = String::new();
    io::stdin().read_to_string(&mut file_contents).expect("Failed to read from stdin");
    let mut words: Vec<&str> = file_contents.trim().split('\n').collect();
    words.sort();
    let result = words.join("\n");
    println!("{}", result);
}