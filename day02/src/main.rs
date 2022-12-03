use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io::Result;

fn main() {
    println!("Day 2: Rock Paper Scissors");
}

fn get_game_score(play: (&str, &str)) -> i32 {
    get_selected_shape_score(play) + get_outcome_score(play)
}

fn get_total_score(lines: Vec<String>) -> i32 {
    let all_tuples: Vec<(&str, &str)> = lines.iter().map(|x| get_game_tuple(x)).collect();
    all_tuples.iter().fold(0, |acc, x| acc + get_game_score(*x))
}

fn get_game_tuple(play: &str) -> (&str, &str) {
    let mut split = play.split_whitespace();
    // (split.next().unwrap(), convert_shape(split.next().unwrap()))
    let opponent_shape = split.next().unwrap();
    (opponent_shape, (get_shape_by_expected_outcome((opponent_shape, split.next().unwrap()))))
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_input_file_name() -> &'static str {
  "mock.txt"
}

// X lose, Y draw Z win
fn get_shape_by_expected_outcome(play: (&str, &str)) -> &'static str {
    match play {
       ("A", "Y") => "A",
       ("A", "X") => "C",
       ("A", "Z") => "B",
       ("B", "Y") => "B",
       ("B", "X") => "A",
       ("B", "Z") => "C",
       ("C", "Y") => "C",
       ("C", "X") => "B",
       ("C", "Z") => "A",
       (_, _) => "",
    }
}

// (0 if you lost, 3 if the round was a draw, and 6 if you won).
fn get_outcome_score(play: (&str, &str)) -> i32 {
    match play {
       ("A", "A") => 3,
       ("B", "B") => 3,
       ("C", "C") => 3,
       ("A", "B") => 6,
       ("C", "A") => 6,
       ("B", "C") => 6,
       ("C", "B") => 0,
       ("B", "A") => 0,
       ("A", "C") => 0,
       (_, _) => 0,
    }
}

fn convert_shape(shape: &str) -> &str {
    match shape {
        "X" => "A", // Rock
        "Y" => "B", // Paper
        "Z" => "C", // Scissors
        _   => ""
    }
}

//    score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
fn get_selected_shape_score(play: (&str, &str)) -> i32 {
    let score = match play {
       (_, "A") => 1,
       (_, "B") => 2,
       (_, "C") => 3,
        _  => 0,
    };
    score
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_total_score() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(error) => panic!("Error getting line {:?}", error),
        };
        let total_score = get_total_score(lines);
        assert_eq!(total_score, 15);
    }

    #[test]
    fn test_get_shape_by_expected_outcome() {
        let game_01 = ("A", "Y");
        let game_02 = ("B", "X");
        let game_03 = ("C", "Z");
        assert_eq!(get_shape_by_expected_outcome(game_01), "A");
        assert_eq!(get_shape_by_expected_outcome(game_02), "A");
        assert_eq!(get_shape_by_expected_outcome(game_03), "A");
    }

    #[test]
    fn test_get_game_score() {
        let game_01 = ("A", "B");
        let game_02 = ("B", "C");
        let game_03 = ("C", "B");
        let game_04 = ("A", "A");
        assert_eq!(get_game_score(game_01), 8);
        assert_eq!(get_game_score(game_02), 9);
        assert_eq!(get_game_score(game_03), 2);
        assert_eq!(get_game_score(game_04), 4);
    }

    #[test]
    fn test_get_outcome_score() {
        let game_01 = ("A", "B");
        let game_02 = ("B", "C");
        let game_03 = ("C", "B");
        let game_04 = ("A", "A");
        assert_eq!(get_outcome_score(game_01), 6);
        assert_eq!(get_outcome_score(game_02), 6);
        assert_eq!(get_outcome_score(game_03), 0);
        assert_eq!(get_outcome_score(game_04), 3);
    }

    #[test]
    fn test_get_selected_shape_score() {
        let game_01 = ("C", "A");
        let game_02 = ("A", "B");
        let game_03 = ("B", "C");
        assert_eq!(get_selected_shape_score(game_01), 1);
        assert_eq!(get_selected_shape_score(game_02), 2);
        assert_eq!(get_selected_shape_score(game_03), 3);
    }

    #[test]
    fn test_convert_shape() {
        let shape_01 = "Y";
        let shape_02 = "X";
        let shape_03 = "Z";
        assert_eq!(convert_shape(shape_01), "B");
        assert_eq!(convert_shape(shape_02), "A");
        assert_eq!(convert_shape(shape_03), "C");
    }


    #[test]
    fn test_get_game_tuple() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(error) => panic!("Error getting line {:?}", error),
        };
        let selected_line = &lines[0 as usize];
        let game_tuple = get_game_tuple(selected_line);
        assert_eq!(game_tuple, ("A", "B"));
    }

    #[test]
    fn test_get_lines_from_file() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(error) => panic!("Error getting line {:?}", error),
        };
        let selected_line = &lines[0 as usize];
        assert_eq!(selected_line, "A Y");
    }

}
