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



    let mut sum_of_possible_game_ids = 0;
    let bag_limit = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for line in reader.lines() {
        let game = line?;

        if is_game_possible(game.as_str(), &bag_limit) {
            println!("game_id: {:?} {:?}", game.as_str(), game.split_whitespace().nth(1).expect("not valid").split(":").nth(0).unwrap());

            if let Some(id) = game.split_whitespace().nth(1).expect("not valid").split(":").nth(0) {
                sum_of_possible_game_ids += id.parse::<i32>().unwrap_or(0);
            }
        }
    }

    println!("Sum of IDs of possible games: {}", sum_of_possible_game_ids);

    Ok(())
}



fn is_game_possible(game: &str, bag_limit: &HashMap<&str, usize>) -> bool {
    let mut color_count_map: HashMap<&str, i32> = HashMap::new();

    let sets = game.split(':').nth(1).unwrap_or_default().split(';');

    let mut vec_of_maps: Vec<HashMap<&str, i32>> = Vec::new();

    for set in sets {
        let mut map: HashMap<&str, i32> = HashMap::new();
        for color_info in set.trim().split(',') {
            let parts: Vec<&str> = color_info.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let count = parts[0].parse::<i32>().unwrap_or(0);
                let color = parts[1];
                map.insert(color, count);
            }
        }
        vec_of_maps.push(map);
    }


    for set in vec_of_maps {
        for (color, count) in set {
           
            if let Some(&limit) = bag_limit.get(&color) {
                println!("color,count,limit: {:?} {:?} {:?}", &color,count,limit as i32);
                if (count > limit as i32) {
                    eprintln!("not possible");

                    return false;

                }
            }
            eprintln!("_____________");

        }
        eprintln!("==========================================");

    }

return true;

}