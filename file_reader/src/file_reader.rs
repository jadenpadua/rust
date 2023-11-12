use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    // Collect file path from arguments
    let file_path: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Error: Please only input the file path as your first argument"),
    };
    // Open file
    let file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };
    // Read file line by line
    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => match error.kind() {
                std::io::ErrorKind::InvalidData => {
                    panic!("Invalid data: {}", error)
                }
                _ => {
                    panic!("Error reading file: {}", error)
                }
            },
        }
    }
}
