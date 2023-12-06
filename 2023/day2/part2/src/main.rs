use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;
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

    let mut total_power: i64 = 0;



    for line in reader.lines() {
        let game = line?;
        //println!("GAME: {:?}", game);

        let min_cubes = find_min_cubes(game.as_str());
        println!("Min cubes required: {:?}", min_cubes);

        let power = min_cubes.values().product::<i64>();
        total_power += power;

    }

    println!("Total power of all games: {}", total_power);

    Ok(())
}


fn find_min_cubes(game: &str) -> HashMap<&str, i64> {
    let mut min_cubes: HashMap<&str, i64> = HashMap::new();

    let sets = game.split(":").nth(1).unwrap().split(';');

    for set in sets {

        let mut set_counts: HashMap<&str, i64> = HashMap::new();
        for cube_info in set.trim().split(',') {

            let parts: Vec<&str> = cube_info.trim().split_whitespace().collect();

            if parts.len() == 2 {
                let count = parts[0].parse::<i64>().unwrap_or(0);
                let color = parts[1];
                *set_counts.entry(color).or_insert(0) += count;
            }
        }


        for (color, &count) in set_counts.iter() {
            min_cubes.entry(color).and_modify(|e| *e = i64::max(*e, count)).or_insert(count);
        }
    }

    min_cubes
}