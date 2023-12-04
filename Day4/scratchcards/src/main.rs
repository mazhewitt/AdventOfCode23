use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scratchcards::scratchcard_game::ScratchCard;
fn main() {
    let matches = App::new("Scratchcard Calculator")
        .version("1.0")
        .author("Mazda Hewitt")
        .about("Calculates total points from a file of scratchcards")
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Sets the input file to use")
            .takes_value(true)
            .required(true))
        .get_matches();

    let filename = matches.value_of("file").unwrap();

    if let Ok(lines) = read_lines(filename) {
        let mut total_points = 0;
        for line in lines.flatten() {
            if let Some((winning, scratch)) = parse_line(&line) {
                let card = ScratchCard::from_string(&scratch).unwrap();
                total_points += card.calculate_points(&winning);
            }
        }
        println!("Total points: {}", total_points);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> Option<(Vec<i32>, String)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 2 {
        return None;
    }
    let winning_numbers = parts[0]
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();
    Some((winning_numbers, parts[1].trim().to_string()))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_points_from_input_line_by_line() {
        let fake_file_content = "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let mut total_points = 0;
        for line in fake_file_content.lines() {
            if let Some((winning, scratch)) = parse_line(line) {
                let card = ScratchCard::from_string(&scratch).unwrap();
                total_points += card.calculate_points(&winning);
            }
        }

        assert_eq!(total_points, 13, "Total points should be 13.");
    }
}