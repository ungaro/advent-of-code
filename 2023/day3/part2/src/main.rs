use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct NumberPosition {
    number: i32,
    start: usize,
    end: usize,
    row: usize,
}

impl fmt::Display for NumberPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{number: {}, start: {}, end: {}, row: {}}}",
            self.number, self.start, self.end, self.row
        )
    }
}

#[derive(Debug)]
struct SymbolPosition<'a> {
    symbol: &'a str,
    start: usize,
    end: usize,
    row: usize,
}

impl fmt::Display for SymbolPosition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{symbol: '{}', start: {}, end: {}, row: {}}}",
            self.symbol, self.start, self.end, self.row
        )
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

    println!("rows:cols {:?} : {:?}", rows, cols);

    // match any characters that are not 0-9 and .
    let re = Regex::new(r"[^\d\.]").unwrap();
    let symbols = re
        .find_iter(schematic[1])
        .map(|mat| SymbolPosition {
            symbol: mat.as_str(),
            start: mat.start(),
            end: mat.end() - 1, // Adjust to make end position inclusive
            row: 1,
        })
        .collect::<Vec<_>>();

    // Print results
    for symbol in symbols {
        println!("{} ", symbol);
    }

    let mut symbol_positions = HashMap::new();
    let re = Regex::new(r"\*").unwrap();
    let mut positions = Vec::new();
    for mat in re.find_iter(schematic[0]) {
        positions.push(mat.start());
    }
    if !positions.is_empty() {
        symbol_positions.insert(0, positions);
    }
    //println!("symbol_positions {:?} ", symbol_positions);
    //println!("schematic_len {:?} ", schematic.len());

    for (i, row) in schematic.iter().enumerate() {
        println!("i row {:?} {:?}", i, row);
        let mut positions = Vec::new();
        //println!("symbol_LEN {:?} ", i + 1);
        if (schematic.len() > i + 1) {
            //  println!("symbol_LEN {:?} ", i + 1);

            for mat in re.find_iter(schematic[i + 1]) {
                positions.push(mat.start());
            }
            if !positions.is_empty() {
                symbol_positions.insert(i + 1, positions);
            }
        }
        //println!("symbol_positions {:?} ", symbol_positions);

        // match any characters that are  0-9 and infinitely in between (means numbers like 2834 21 or 2837459823)
        let re = Regex::new(r"\d+").unwrap();
        let numbers = re
            .find_iter(row)
            .map(|mat| NumberPosition {
                number: mat.as_str().parse::<i32>().unwrap(),
                start: mat.start(),
                end: mat.end() - 1, // Adjust to make end position inclusive
                row: i,
            })
            .collect::<Vec<_>>();

        // Print results
        for num in numbers {
            //println!("Numbers: {}", num);
            if is_adjacent_to_symbol(num.clone(), symbol_positions.clone()) {
                //eprintln!("true");
                println!("NUMBER {:?}", num.number);

                // If it's a number and adjacent to a symbol, add to sum
                sum += num.number as i32;
            }
        }
    }

    sum
}

fn is_adjacent_to_symbol(
    num: NumberPosition,
    symbol_positions: HashMap<usize, Vec<usize>>,
) -> bool {
    /*
        println!(
            "numnum {:?} {:?} {:?} {:?} ",
            num.number, num.start, num.end, num.row
        );
    */

    if (num.row > 0) {
        match symbol_positions.get(&(num.row - 1)) {
            Some(value) => {
                let is_within_range = value.iter().any(|&number| {
                    let start = if num.start > 0 {
                        num.start - 1
                    } else {
                        num.start
                    };

                    number >= start && number <= num.end + 1
                });
                if is_within_range {
                    //println!("At least one number is in the range.");
                    return true;
                }

                // println!("Value for key '{:?}': {:?}", num.row-1, value)
            }
            None => (),
        }
    }
    match symbol_positions.get(&num.row) {
        Some(value) => {
            let is_within_range = value.iter().any(|&number| {
                let start = if num.start > 0 {
                    num.start - 1
                } else {
                    num.start
                };

                number >= start && number <= num.end + 1
            });
            if is_within_range {
                //println!("At least one number is in the range.");
                return true;
            }

            //    println!("Value for key '{:?}': {:?}", num.row, value)
        }
        None => (),
    }

    match symbol_positions.get(&(num.row + 1)) {
        Some(value) => {
            let is_within_range = value.iter().any(|&number| {
                let start = if num.start > 0 {
                    num.start - 1
                } else {
                    num.start
                };

                number >= start && number <= num.end + 1
            });
            if is_within_range {
                //println!("At least one number is in the range.");
                return true;
            }

            // println!("Value for key '{:?}': {:?}", num.row+1, value)
        }
        None => (),
    }
  
    eprintln!(
        "===================================================================================="
    );


    false
}
