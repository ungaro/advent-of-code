use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("../input.txt");

    // Check if the file exists
    if !path.exists() {
        eprintln!("Error: 'input.txt' not found in the current directory.");
        return Ok(());
    }

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut total_points = 0;

    for line in reader.lines() {
        let line = line?;
        total_points += calculate_card_points(&line);
    }

    println!("Total points: {}", total_points);

    Ok(())
}

fn calculate_card_points(card: &str) -> i32 {
    let parts: Vec<&str> = card.split('|').collect();
    let winning_numbers: Vec<i32> = parts[0]
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();
    let my_numbers: Vec<i32> = parts[1]
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    let my_numbers_hash: HashSet<_> = my_numbers.into_iter().collect();

    let count = winning_numbers
        .iter()
        .filter(|&item| my_numbers_hash.contains(item))
        .count();

    let number: i32 = 2; 
    let mut square = 0;
    if count > 0 {
        let exponent = (count - 1) as u32; // Convert to u32 for pow
        square = number.pow(exponent); // Raise to the power
    }

    println!("2 square count-1  {:?}", square);

    square
}
