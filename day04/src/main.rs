use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::path::Path;
use std::io::Result;

// pairs  look similar to lines
// a Point - Line approach can be used
// then check if one contains the other

// get section_pair
// parse section_range into section_array 
// find if array_a contains array_b and b-a -> boolean
// count contained arrays
fn main() {
    println!("--- Day 4: Camp Cleanup ---");
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_input_file_name() -> &'static str {
  "mock.txt"
}

fn get_section_range(selected_line: &str) -> std::ops::Range<i32> {
    let mut split = selected_line.split_terminator(",");
    let a = split.next().unwrap();
    let b = split.next().unwrap();
    let mut a_split = a.split_terminator("-");
    let a_parsed = (a_split.next().unwrap(), a_split.next().unwrap());
    let mut b_split = b.split_terminator("-");
    // let b_parsed: (i32, i32) = (b_split.next().unwrap().parse().unwrap(), b_split.next().unwrap().parse().unwrap());
    let b_parsed: std::ops::Range<i32> = (b_split.next().unwrap().parse().unwrap()..b_split.next().unwrap().parse().unwrap());
    println!("a_parsed: {:?}", a_parsed);
    println!("b_parsed: {:?}", b_parsed);

    2..3
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_part_01_result() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(error) => panic!("Error getting line {:?}", error),
        };

        let selected_line = &lines[0 as usize];
        assert_eq!(selected_line, "2-4,6-8");

        let range_a = get_section_range(selected_line); 
        assert_eq!(range_a, (2..4));
    }
}
