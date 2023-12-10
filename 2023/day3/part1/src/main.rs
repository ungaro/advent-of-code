use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::fmt;



#[derive(Debug)]
struct NumberPosition {
    number: i32,
    start: usize,
    end: usize,
}

impl fmt::Display for NumberPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{number: {}, start: {}, end: {}}}", self.number, self.start, self.end)
    }
}


fn main() -> io::Result<()> {

    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    let path = Path::new("../input2.txt");

    // Check if the file exists
    if !path.exists() {
        eprintln!("Error: 'input.txt' not found in the current directory.");
        return Ok(());
    }

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }
    //println!("lines: {:?}", lines);

    let lines_str: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();

    let sum = sum_part_numbers(&lines_str);
    println!("Sum of part numbers: {}", sum);
    Ok(()) 

}

fn sum_part_numbers(schematic: &[&str]) -> i32 {
    let mut sum = 0;
    let rows = schematic.len();
    let cols = schematic[0].len(); // assuming all rows are of equal length

    println!("rows:cols {:?} : {:?}", rows,cols);



    let re = Regex::new(r"\d+").unwrap();
    let numbers = re.find_iter(schematic[0])
                    .map(|mat| NumberPosition {
                        number: mat.as_str().parse::<i32>().unwrap(),
                        start: mat.start(),
                        end: mat.end() - 1, // Adjust to make end position inclusive
                    })
                    .collect::<Vec<_>>();

    // Print results
    for num in numbers {
        println!("{}", num);
    }





    for (i, row) in schematic.iter().enumerate() {
        println!("i row {:?} {:?}",i,row);

        for (j, &ch) in row.as_bytes().iter().enumerate() {
            //println!("ch {:?}",ch);

            if ch.is_ascii_digit() {
                // Check adjacent characters for symbols
                if is_adjacent_to_symbol(schematic, i, j, rows, cols) {
                    // If it's a number and adjacent to a symbol, add to sum
                    sum += ch as i32 - '0' as i32;
                }
            }
        }
    }

    sum
}

fn is_adjacent_to_symbol(schematic: &[&str], i: usize, j: usize, rows: usize, cols: usize) -> bool {
    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
    let dy = [-1, -1, -1, 0, 0, 1, 1, 1];
    //println!("schematic,i,j,rows,cols {:?} : {:?}: {:?}: {:?}: {:?}",schematic,i,j,rows,cols);
    //eprintln!("====================================================================================");

    for k in 0..8 {
        let new_i = i as i32 + dx[k];
        let new_j = j as i32 + dy[k];

        if new_i >= 0 && new_i < rows as i32 && new_j >= 0 && new_j < cols as i32 {
            let adjacent_char = schematic[new_i as usize].as_bytes()[new_j as usize] as char;
            if adjacent_char != '.' && !adjacent_char.is_ascii_digit() {
                return true;
            }
        }
    }

    false
}