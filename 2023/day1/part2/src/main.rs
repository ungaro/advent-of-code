use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;
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

    // Mapping of spelled-out digits to their numeric form
    let digit_map: HashMap<&str, u32> = [
        ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ].iter().cloned().collect();

    for line in reader.lines() {
        let line = line?;
        let digits = extract_digits(&line, &digit_map);

            let first_digit = digits[0];
            let last_digit = digits[digits.len() - 1];
            let number = first_digit * 10 + last_digit;
            println!("line - digits - number  {:?} : {:?} : {:?}",line,digits,number);

            sum += number;

    }

    println!("Total sum of calibration values: {}", sum);

    Ok(())
}

// Function to extract digits (including spelled-out digits) from a line
fn extract_digits(line: &str, digit_map: &HashMap<&str, u32>) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut current_word = String::new();

    for c in line.chars() {
        if c.is_alphabetic() {
            current_word.push(c);

//            println!("current word  {:?}",current_word);


match contains_key_from_map(current_word.as_str(), &digit_map) {
    Some(key) => {
        if let Some(&number) = digit_map.get(key) {
            digits.push(number);
        }
        if current_word.chars().count() >= 2 {
           current_word= current_word.chars().rev().take(2).collect::<String>().chars().rev().collect()
        }

    },
    None => (),
}
        } else if c.is_digit(10) {
            // If there's a partial word not matching a spelled-out digit, clear it
            if !digit_map.contains_key(current_word.as_str()) {
                current_word.clear();
            }
            digits.push(c.to_digit(10).unwrap());
        }
    }

    // Check for a valid spelled-out digit at the end of the line
    if !current_word.is_empty() && digit_map.contains_key(current_word.as_str()) {
        if let Some(&number) = digit_map.get(current_word.as_str()) {
            digits.push(number);
        }
    }

    digits
}

fn contains_key_from_map<'a>(s: &'a str, map: &'a HashMap<&'a str, u32>) -> Option<&'a str> {
    for key in map.keys() {
        if s.contains(key) {
            return Some(key);
        }
    }
    None
}
