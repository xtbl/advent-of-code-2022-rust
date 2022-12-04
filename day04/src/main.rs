use std::fs::File;
use std::io::{BufRead, BufReader};
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

fn is_range_contained(range_a: std::ops::RangeInclusive<i32>, range_b: std::ops::RangeInclusive<i32>) -> bool {
    let mut a = range_a.to_owned();
    let mut b = range_b.to_owned();
    b.all(|item_b| a.contains(&item_b)) || a.all(|item_a| b.contains(&item_a)) 
}

fn is_range_contained_array(range_a: Vec<i32>, range_b: Vec<i32>) -> bool {
    let mut a = range_a.to_owned();
    let mut b = range_b.to_owned();
    b.iter().all(|item_b| a.contains(&item_b)) || a.iter().all(|item_a| b.contains(&item_a)) 
}

fn is_range_overlap_array(range_a: Vec<i32>, range_b: Vec<i32>) -> bool {
    let mut a = range_a.to_owned();
    let mut b = range_b.to_owned();
    b.iter().any(|item_b| a.contains(&item_b)) || a.iter().any(|item_a| b.contains(&item_a)) 
}

fn get_section_range(range_line: &str) -> std::ops::RangeInclusive<i32> {
    let mut range_split = range_line.split_terminator("-");
    let start = range_split.next().unwrap().parse().unwrap();
    let end = range_split.next().unwrap().parse().unwrap();
    start..=end
    // range_split.next().unwrap().parse().unwrap()..range_split.next().unwrap().parse().unwrap()
}

fn get_section_range_array(range_line: &str) -> Vec<i32> {
    let mut range_split = range_line.split_terminator("-");
    let start = range_split.next().unwrap().parse().unwrap();
    let end = range_split.next().unwrap().parse().unwrap();
    let mut vec: Vec<i32> = (start..=end).collect();
    vec
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
        // assert_eq!(selected_line, "2-4,6-8");

        let mut split = selected_line.split_terminator(",");

        let range_a = get_section_range(split.next().unwrap()); 
        let range_b = get_section_range(split.next().unwrap()); 
        // assert_eq!(range_a, (2..=4));
        // assert_eq!(range_b, (6..=8));

        let mut split = selected_line.split_terminator(",");
        let range_a = get_section_range_array(split.next().unwrap()); 
        let range_b = get_section_range_array(split.next().unwrap()); 
        // assert_eq!(range_a, [2,3,4]);
        // assert_eq!(range_b, [6,7,8]);

        assert_eq!(is_range_contained_array(vec![2,3,4],vec![1,2,3,4,5,6,7,8]), true);
        assert_eq!(is_range_contained_array(vec![2,3,4],vec![6,7,8]), false);
        assert_eq!(is_range_contained_array(vec![2,3],vec![4,5]), false);


        let is_range_contained_a = is_range_contained(2..=10, 4..=5);
        let is_range_contained_b = is_range_contained(1..=2, 4..=5);
        let is_range_contained_c = is_range_contained(2..=8, 3..=7);
        let is_range_contained_d = is_range_contained(2..=8, 3..=7);

        assert_eq!(is_range_contained_a, true);
        assert_eq!(is_range_contained_b, false);
        assert_eq!(is_range_contained_c, true);
        assert_eq!(is_range_contained(301..=700, 300..=700), true);
        assert_eq!(is_range_contained(1..=1, 1..=1), true);
        assert_eq!(is_range_contained(1..=1, 1..=2), false); // this is why range contains doesn't work
        assert_eq!(is_range_contained_array(vec![1],vec![1,2]), true);

        // Using range
        // let pairs_contained: usize = lines.iter().map(|line| {
        //     let mut split = line.split_terminator(",");
        //     let range_a = get_section_range(split.next().unwrap()); 
        //     let range_b = get_section_range(split.next().unwrap()); 
        //     is_range_contained(range_a, range_b)
        // }).filter(|x| *x).count();
        // println!("pairs_contained: {:?}", pairs_contained);

        // assert_eq!(pairs_contained, 2);

        // Using arrays
        let pairs_contained_array: usize = lines.iter().map(|line| {
            let mut split = line.split_terminator(",");
            let range_a = get_section_range_array(split.next().unwrap()); 
            let range_b = get_section_range_array(split.next().unwrap()); 
            is_range_overlap_array(range_a, range_b)
        }).filter(|x| *x).count();
        println!("pairs_contained: {:?}", pairs_contained_array);

        assert_eq!(pairs_contained_array, 4);
    }
}
