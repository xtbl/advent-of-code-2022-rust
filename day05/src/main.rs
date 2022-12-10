
//     [D]    
// [N] [C]    
// [Z] [M] [P]

//  1   2   3 

// get number of columns: line char amount / 4 - 1
// get chunks of 3 chars until end, take into account spaces
// parsing: group of 3 spaces, 1 space, [char] 
// find if empty line: this is the separator to move instructions
// if empty line change to read_instructions mode
//     no need to complicated parsing [] if chars can be calculated from 
//         numCol 
// read_instructions mode
// parse
// move 1 from 2 to 1
// move -> amount - from -> i
// move crate_amount, origin, destiny




// parse crates into stacks -> parse_stacks
    // build stacks
// empty line separates
// parse moves -> map to (crate_amount, origin, destiny)
    // build moves -> (crate_amount, origin, destiny)
// move_crates
// stacks vec,  moves are array push pop
// Vec<Box<Vec<char>>>> or Hashmap<position, value> 
// or Vec<string> as each string would push pop chars
// Hashmap<(stack, position), char>
// stack number, position in stack (amount of same stack + 1), char
// move 1 from 2 to 1
// move crate_amount, origin, destiny

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io::Result;
use nom::{
    character::complete::alpha0,
    character::complete::alpha1,
    character::complete::char,
    sequence::delimited,
    error::Error,
    error::ParseError,
    combinator::value,
    sequence::tuple,
    bytes::complete::{tag, take_until},
    bytes::complete::take,
    bytes::complete::take_while,
    IResult
};

// use std::slice::Chunks;
// use itertools::Itertools;

fn main() {
    println!("--- Day 5: Supply Stacks ---");
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_input_file_name() -> &'static str {
  "mock.txt"
}
// perhaps nom take(4) can be used here too
fn parse_line_into_chunks(line: &str) -> Vec<String> {
    let chars_per_column = 4 ;
    line.chars()
        .collect::<Vec<char>>()
        .chunks(chars_per_column)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
}


// tag take until tag
// https://blog.adamchalmers.com/nom-char
// https://github.com/geal/nom/blob/main/doc/choosing_a_combinator.md
// https://stackoverflow.com/questions/57681018/catch-string-between-tags-with-nom-delimited
// use this as reference for tags [D] https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#whitespace
// TODO: read https://github.com/Geal/nom/blob/main/doc/making_a_new_parser_from_scratch.md
fn parser(input: &str) -> IResult<&str, &str> {
    tag("[")(input)
}

fn parserTake(input: &str) -> IResult<&str, &str> {
    take(4usize)(input)
}

// fn parser_crates<'a, E: ParseError<&'a str>>(input: &str) -> IResult<(&str, &str), nom::Err<E>> {
//     // FIX: change to alpha
//     delimited(tag("["), take_until(alpha0), tag("]"))
// }

// fn parser_crate<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &str, E> {
//     delimited(
//         tag("["),
//         take_while(char::is_alphanumeric),
//         tag("]"),
//     )(i)
// }

// pub fn pinline_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
//   value(
//     (), // Output is thrown away.
//     tuple((
//       tag("["),
//       take_until("])"),
//       tag("]")
//     ))
//   )(i)
// }

fn parse_string_to_crate(input_chunk: &str) -> Option<String> {
    let input = input_chunk.clone();
    let trimmed = input.trim();
    let remove_extra_chars = trimmed.replace(&['[', ']'][..], "");
    println!("--- remove_extra_chars {:?}", remove_extra_chars);
    match remove_extra_chars.is_empty() {
        false => Some(String::from(remove_extra_chars)),
        true => None
    }
}

fn fill_crates(lines: Vec<String>) -> Vec<Vec<String>> {
    vec![vec![String::from("A"), String::from("B")]]
}

// TODO: parse line into stacks
// iterate vec with chunks
// if Some push item, else skip position

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_part_01_result() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(e) => panic!("Error getting line {:?}", e),
        };

        assert_eq!("[N] [C]    ", &lines[1 as usize]);
        assert_eq!("    [D]    ", &lines[0 as usize]);

        assert_eq!(vec!["    ", "[D] ", "   "], parse_line_into_chunks(&lines[0 as usize]));

        // parse chunks to columns
        assert_eq!(None, parse_string_to_crate("    "));
        assert_eq!(Some(String::from("D")), parse_string_to_crate("[D] "));
        assert_eq!(None, parse_string_to_crate("   "));
        assert_eq!(Some(String::from("V")), parse_string_to_crate("[V] "));

    }

    // #[test]
    // fn test_parser<'a, E: ParseError<&'a str>>() {
    //     let input =  "[D] ".trim();
    //     let parsed: IResult<&'a str, (), E> = pinline_comment(input);
    //     // println!("--- parsed {:?}", parsed); 
    //     assert_eq!(Ok(("1c", "sdfsdf")), Ok(("1c", "D")));
    //     // assert_eq!(parser_crate(" [D] ".trim()), Ok(("1c", "D")));

    // }

    // #[test]
    // fn test_parser<'a, E: ParseError<&'a str> + std::cmp::PartialEq + std::fmt::Debug>() {
    //     let input =  "[D] ".trim();
    //     let parsed = parser_crate::<'a,E>(input);
    //     assert_eq!(parsed, Ok(("1c", "D")));
    //     // assert_eq!(parser_crate(" [D] ".trim()), Ok(("1c", "D")));

    // }
}
