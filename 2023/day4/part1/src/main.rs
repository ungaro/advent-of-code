use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Print the current working directory
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

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
    let winning_numbers: Vec<i32> = parts[0].split_whitespace().filter_map(|num| num.parse().ok()).collect();
    let my_numbers: Vec<i32> = parts[1].split_whitespace().filter_map(|num| num.parse().ok()).collect();

    let mut points = 0;
    let mut multiplier = 1;

    for num in my_numbers {
        if winning_numbers.contains(&num) {
            points += multiplier;
            multiplier *= 2;
        }
    }

    points
}