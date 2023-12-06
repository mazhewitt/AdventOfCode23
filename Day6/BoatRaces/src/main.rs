use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn calculate_ways_to_win(times: &[i32], records: &[i32]) -> i32 {
    let mut total_ways = 1;

    for (&time, &record) in times.iter().zip(records.iter()) {
        let mut ways_to_win = 0;

        for hold_time in 0..time {
            let distance = hold_time * (time - hold_time);
            if distance > record {
                ways_to_win += 1;
            }
        }

        total_ways *= ways_to_win;
    }

    total_ways
}

fn calculate_ways_to_win_single_race(time: i64, record: i64) -> i64 {
    let mut ways_to_win = 0;

    for hold_time in 0..time {
        let distance = hold_time * (time - hold_time);
        if distance > record {
            ways_to_win += 1;
        }
    }

    ways_to_win
}



fn read_input_file<P: AsRef<Path>>(filename: P) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().filter_map(|line| line.ok()).collect();

    if lines.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid number of lines"));
    }

    let times = lines[0].split_whitespace()
        .skip(1) // Skip the "Time:" label
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances = lines[1].split_whitespace()
        .skip(1) // Skip the "Distance:" label
        .filter_map(|s| s.parse().ok())
        .collect();

    Ok((times, distances))
}


fn read_input_file_single_race<P: AsRef<Path>>(filename: P) -> io::Result<(i64, i64)> {

        let file = File::open(filename)?;
        let lines: Vec<String> = io::BufReader::new(file).lines().filter_map(|line| line.ok()).collect();

        if lines.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid number of lines"));
        }

        let time_s_vec:Vec<&str> = lines[0].split_whitespace()
            .skip(1) // Skip the "Time:" label
            .collect();
        let time = time_s_vec.join("").parse::<i64>().unwrap();
        let distance_s_vec:Vec<&str> = lines[1].split_whitespace()
            .skip(1) // Skip the "Time:" label
            .collect();
        let distance = distance_s_vec.join("").parse::<i64>().unwrap();

        Ok((time, distance))
}


fn main() -> io::Result<()> {
    let (times, distances) = read_input_file("input_file.txt")?;

    let result = calculate_ways_to_win(&times, &distances);
    println!("Number of ways to win multiple races: {}", result);

    let (time, distance) = read_input_file_single_race("input_file.txt")?;
    let result2 = calculate_ways_to_win_single_race(time, distance);
    println!("Number of ways to win a single race: {}", result2);
    Ok(())
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_ways_to_win() {
        let times = vec![7, 15, 30];
        let records = vec![9, 40, 200];
        let expected = 288; // The expected result based on the example
        assert_eq!(calculate_ways_to_win(&times, &records), expected);
    }

    #[test]
    fn test_calculate_ways_to_win_single_race() {
        let time = 71530;
        let distance = 940200;
        let expected_ways_to_win = 71503; // Expected result based on the example

        assert_eq!(
            calculate_ways_to_win_single_race(time, distance),
            expected_ways_to_win
        );
    }

    #[test]
    fn test_calculate_ways_to_win_single_race2() {
        let time = 40929790;
        let distance = 215106415051100;
        let expected_ways_to_win = 28545089; // Expected result based on the example

        assert_eq!(
            calculate_ways_to_win_single_race(time, distance),
            expected_ways_to_win
        );
    }

    #[test]
    fn test_read_input_single_race(){
        let expected_time = 40929790;
        let expected_distance = 215106415051100;
        let (time, distance) = read_input_file_single_race("input_file.txt").unwrap();
        assert_eq!(time, expected_time);
        assert_eq!(distance, expected_distance);
    }

}

