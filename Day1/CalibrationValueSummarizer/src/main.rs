use clap::{App, Arg};
use std::fs;
use std::collections::HashMap;
use regex::Regex;
fn main() {
    let matches = App::new("Calibration Summarizer")
        .version("1.0")
        .author("Your Name")
        .about("Calculates the sum of calibration values from a file")
        .arg(
            Arg::with_name("file")
                .help("The file to process")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file_name = matches.value_of("file").unwrap();
    let lines = read_lines_from_file(file_name);
    let sum = sum_calibration_values(lines);

    println!("The sum of the calibration values is: {}", sum);
}


fn read_lines_from_file(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn sum_calibration_values(lines: Vec<String>) -> usize {
    lines.iter()
        .map(|line| extract_calibration_value(line))
        .sum()
}

fn extract_calibration_value(input: &str) -> usize {
    let first_number = find_first_number(input).unwrap_or(0);
    let last_number = find_last_number(input).unwrap_or(0);
    first_number * 10 + last_number
}


fn find_first_spelled_out_number(input: &str) -> Option<(usize, usize)> {
    let words_to_numbers = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ];

    words_to_numbers
        .iter()
        .filter_map(|(word, number)| {
            input.find(word).map(|index| (index, *number))
        })
        .min_by_key(|&(index, _)| index)
}

fn find_first_digit(input: &str) -> Option<(usize, usize)> {
    input.char_indices()
        .filter_map(|(index, ch)| {
            ch.to_digit(10)
                .map(|digit| (index, digit as usize))
        })
        .next()
}

fn find_first_number(input: &str) -> Option<usize> {
    let spelled_out = find_first_spelled_out_number(input);
    let digit = find_first_digit(input);

    [spelled_out, digit]
        .iter()
        .filter_map(|&option| option)
        .min_by_key(|&(index, _)| index)
        .map(|(_, number)| number)
}


fn find_last_spelled_out_number(input: &str) -> Option<(usize, usize)> {
    let words_to_numbers = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ];

    words_to_numbers
        .iter()
        .filter_map(|(word, number)| {
            input.rfind(word).map(|index| (index, *number))
        })
        .max_by_key(|&(index, _)| index)
}

fn find_last_digit(input: &str) -> Option<(usize, usize)> {
    input.char_indices()
        .rev()
        .filter_map(|(index, ch)| {
            ch.to_digit(10)
                .map(|digit| (index, digit as usize))
        })
        .next()
}

fn find_last_number(input: &str) -> Option<usize> {
    let spelled_out = find_last_spelled_out_number(input);
    let digit = find_last_digit(input);

    [spelled_out, digit]
        .iter()
        .filter_map(|&option| option)
        .max_by_key(|&(index, _)| index)
        .map(|(_, number)| number)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_calibration_value_with_words() {
        let examples = vec![
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

        for (line, expected) in examples {
            assert_eq!(extract_calibration_value(line), expected, "Failed on line: {}", line);
        }
    }

    #[test]
    fn test_find_first_spelled_out_number() {
        let input = "h1twofour434";
        let expected = Some((2, 2)); // "two" starts at index 3
        assert_eq!(find_first_spelled_out_number(input), expected);
    }

    #[test]
    fn test_find_first_digit() {
        let input = "h1twofour434";
        let expected = Some((1, 1)); // "1" is at index 1
        assert_eq!(find_first_digit(input), expected);
    }

    #[test]
    fn test_find_first_number() {
        let input = "h1twofour434";
        let expected = Some(1); // "1" is the first number
        assert_eq!(find_first_number(input), expected);
    }

    #[test]
    fn test_find_last_spelled_out_number() {
        let input = "ftwofdjsif7eight";
        let expected = Some((11, 8)); // "eight" starts at index 11
        assert_eq!(find_last_spelled_out_number(input), expected);
    }

    #[test]
    fn test_find_last_digit() {
        let input = "ftwofdjsif7eight";
        let expected = Some((10, 7)); // "7" is at index 10
        assert_eq!(find_last_digit(input), expected);
    }

    #[test]
    fn test_find_last_number() {
        let input = "ftwofdjsif7eight";
        let expected = Some(8); // "eight" is the last number
        assert_eq!(find_last_number(input), expected);
    }
}