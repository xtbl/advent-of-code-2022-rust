use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io::Result;

fn main() {
    println!("Day 1: Calorie Counting");
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_input_file_name() -> &'static str {
  "input.txt"
}

fn get_most_calories(lines: &Vec<String>) -> i32 {
    let mut accums: Vec<i32> = vec![];
    let mut current_accum: i32 = 0;

    for line in lines {
        if line != "" {
            let line_int: i32 = line.parse().unwrap();
            current_accum = current_accum + line_int;
            println!("--- current_accum");
            println!("{:?}", current_accum);
        } else {
            accums.push(current_accum);
            current_accum = 0;
        }
    }
    println!("--- accums");
    println!("{:?}", accums);

    let most_calories: i32 = match accums.iter().max() {
        Some(val) => *val,
        None => panic!("Error getting most calories"),
    };
    most_calories
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_most_calories() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(error) => panic!("Error getting line {:?}", error),
        };
        println!("--- lines");
        println!("{:?}", lines);
        
        let most_calories = get_most_calories(&lines);
        assert_eq!(most_calories, 24000);
    }

    #[test]
    fn test_get_lines_from_file() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(error) => panic!("Error getting line {:?}", error),
        };
        println!("--- lines");
        println!("{:?}", lines);
        let selected_line = &lines[0 as usize];
        assert_eq!(selected_line, "1000");
    }
}
