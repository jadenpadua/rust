use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    // Collect file path from arguments
    let file_path: &String = args
        .get(1)
        .unwrap_or_else(|| panic!("Error: Please only input the file path as your first argument"));
    let file: File = File::open(file_path).expect("File path should open correctly");
    let reader: BufReader<File> = BufReader::new(file);
    // Read file line by line
    for line in reader.lines() {
        let line: String = line.expect("Line should be read correctly");
        println!("{}", line);
    }
}
