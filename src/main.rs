use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // The first argument is the path tha was used to call the program
    println!("My path is {}.", args[0]);
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    
    let file_path = if args.len() > 1 { &args[1] } else { "non_existent_file.txt" };
    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
