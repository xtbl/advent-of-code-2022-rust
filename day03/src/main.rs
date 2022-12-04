use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::path::Path;
use std::io::Result;

fn main() {
    println!("Day 3: Rucksack Reorganization");
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_input_file_name() -> &'static str {
  "mock.txt"
}

fn get_duplicated_in_both(half_1: &str, half_2: &str) -> char {
    let mut duplicates = vec![];
    for h1 in half_1.chars() {
        for h2 in half_2.chars() {
            if h1 == h2 {
                duplicates.push(h1);
            }

        }
    // duplicates = half_2.chars().filter(|h2| h1 == *h2).collect();
    // println!("--- duplicates: {:?}", duplicates);

    }
    *duplicates.index(0)
}

fn get_char_priority(to_find: char) -> i32 {
    let mut a_z = ('a'..='z').into_iter().collect::<Vec<char>>();
    let A_Z = ('A'..='Z').into_iter().collect::<Vec<char>>();
    a_z.extend(&A_Z);
    a_z.iter().position(|x| *x == to_find).unwrap() as i32 + 1
}

// split in halves
// find item duplicated in both halves
// get item priority
// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
// find duplicated item priority
// sum duplicated items priorities

// part 02
// window 3 to group lines
// find same char in the 3 lines
// find priorities accum


#[cfg(test)]
mod tests {

    use super::*;




    #[test]
    fn test_get_part_01_result() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(error) => panic!("Error getting line {:?}", error),
        };
        
        let accum = lines.iter().fold(0, |acc, line| {
            let (half_1, half_2) = line.split_at(line.len()/2);
            let duplicated = get_duplicated_in_both(half_1, half_2);
            println!("--- duplicated: {:?}", duplicated);
            acc + get_char_priority(duplicated)
        });
        assert_eq!(accum, 157);
    }

    #[test]
    fn test_get_char_priority() {
        assert_eq!(get_char_priority('p'), 16);
        assert_eq!(get_char_priority('L'), 38);
        assert_eq!(get_char_priority('P'), 42);
    }

    #[test]
    fn test_get_duplicated_in_both() {
        let half_1 = "vJrwpWtwJgWr";
        let half_2 = "hcsFMMfFFhFp";
        let duplicated = get_duplicated_in_both(half_1, half_2);
        assert_eq!(duplicated, 'p');
    }

    #[test]
    fn test_split_in_half() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(error) => panic!("Error getting line {:?}", error),
        };
        let selected_line = &lines[0 as usize];
        // assert_eq!(selected_line, "vJrwpWtwJgWrhcsFMMfFFhFp");
        let (half1, half2) = selected_line.split_at(selected_line.len()/2);
        assert_eq!(half1, "vJrwpWtwJgWr");
        assert_eq!(half2, "hcsFMMfFFhFp");

    }




}
