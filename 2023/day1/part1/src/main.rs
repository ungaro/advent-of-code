use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Print the current working directory
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    let path = Path::new("../../data/input.txt");

    // Check if the file exists
    if !path.exists() {
        eprintln!("Error: 'input.txt' not found in the current directory.");
        return Ok(());
    }

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

        let first_digit = digits.first().unwrap().to_digit(10).unwrap();
        let last_digit = digits.last().unwrap().to_digit(10).unwrap();
        let number = first_digit * 10 + last_digit;
        println!("line - number  {:?} : {:?}",line,number);

        sum += number;

    }

    println!("Total sum of calibration values: {}", sum);

    Ok(())
}