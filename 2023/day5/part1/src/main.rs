use std::fs::File;
use std::io::{self, BufRead, BufReader};
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

    let cards: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let total_cards = process_cards(&cards);


    println!("Total number of scratchcards: {}", total_cards);

    Ok(())
}

fn process_cards(cards: &[String]) -> i32 {
    let mut card_copies = vec![1; cards.len()]; // Initialize with 1 copy for each card

    for (i, card) in cards.iter().enumerate() {
        let matches = calculate_matches(card);

        // Distribute matches as copies to subsequent cards
        for j in i+1..std::cmp::min(i+1+matches, cards.len()) {
            card_copies[j] += card_copies[i];
        }
    }

    card_copies.iter().sum()
}

fn calculate_matches(card: &str) -> usize {
    let parts: Vec<&str> = card.split('|').collect();
    let winning_numbers: Vec<&str> = parts[0].split_whitespace().collect();
    let my_numbers: Vec<&str> = parts[1].split_whitespace().collect();

    my_numbers.iter().filter(|num| winning_numbers.contains(num)).count()
}