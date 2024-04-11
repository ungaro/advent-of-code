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

    let races: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let times_line = &races[0];
    let distance_line = &races[1];
    let times: Vec<i32> = times_line.split_whitespace().skip(1).filter_map(|s| s.parse().ok()).collect();
    let distances: Vec<i32> = distance_line.split_whitespace().skip(1).filter_map(|s| s.parse().ok()).collect();

    let races: Vec<(i32, i32)> = times.into_iter().zip(distances.into_iter()).collect();

    let mut total_ways_to_win: i64 = 1;

    for race in races.iter() {
        let ways_to_win = calculate_ways_to_win(race.0, race.1);
        total_ways_to_win *= ways_to_win;
    }

    println!("Total ways to win all races: {}", total_ways_to_win);
    Ok(())

}

fn calculate_ways_to_win(time: i32, record: i32) -> i64 {
    let mut ways_to_win = 0;
    for h in 1..time {
        let distance = h * (time - h);
        if distance > record {
            ways_to_win += 1;
        }
    }
    ways_to_win
}