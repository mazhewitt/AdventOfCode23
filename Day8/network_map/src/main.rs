


use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::collections::HashMap;
use regex::Regex;
use num::integer::lcm;

fn read_puzzle_input(file_path: &str) -> io::Result<(Vec<char>, HashMap<String, Vec<String>>)> {
    let file = File::open(file_path)?;
    let mut lines = io::BufReader::new(file).lines();

    let instructions_str = lines.next().ok_or_else(|| Error::new(ErrorKind::Other, "No instructions found"))??;
    let instructions: Vec<char> = instructions_str.chars().collect();

    let re = Regex::new(r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)").unwrap();
    let mut network = HashMap::new();

    for line in lines.filter_map(Result::ok) {
        if let Some(caps) = re.captures(&line) {
            let node = caps.get(1).unwrap().as_str().to_string();
            let left = caps.get(2).unwrap().as_str().to_string();
            let right = caps.get(3).unwrap().as_str().to_string();
            network.insert(node, vec![left, right]);
        }
    }

    Ok((instructions, network))
}

fn find_escape_steps(instructions: Vec<char>, network: HashMap<String, Vec<String>>) -> u64 {
    let mut current_nodes: Vec<String> = network.keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect();

    let mut steps = 0;
    let mut completed_steps = vec![0; current_nodes.len()];
    let mut finished = vec![false; current_nodes.len()];

    while !finished.iter().all(|&x| x) {
        for (i, node) in current_nodes.iter_mut().enumerate() {
            if finished[i] {
                continue;
            }

            let next_node = &network[node][if instructions[steps % instructions.len()] == 'L' { 0 } else { 1 }];
            *node = next_node.clone();

            if next_node.ends_with('Z') && completed_steps[i] == 0 {
                completed_steps[i] = steps + 1; // Record steps for paths reaching 'Z'
                finished[i] = true;
            }
        }

        steps += 1;
    }

    completed_steps.into_iter().filter(|&x| x > 0).fold(1u64, |a, b| lcm(a, b as u64))
}


fn navigate_to_zzz(instructions: &Vec<char>, network: &HashMap<String, Vec<String>>) -> usize {
    let start = "AAA".to_string();
    let end = "ZZZ".to_string();
    let mut current_node = &start;
    let mut steps = 0;
    while *current_node  != end {
        let intruction_i = // map L to 0 and R to 1
            match instructions[steps%instructions.len()] {
                'L' => 0,
                'R' => 1,
                _ => panic!("Invalid instruction")
            };
        current_node = &network.get(current_node).unwrap()[intruction_i];
        steps += 1;

    }
    steps

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigate_to_zzz_from_file() {

        let file_path = "test_input.txt"; // Update with actual file path
        let (instructions, network) = read_puzzle_input(file_path).expect("Failed to read puzzle input");
        let expected_steps:usize = 2;
        let actual_steps = navigate_to_zzz(&instructions, &network);

        assert_eq!(expected_steps, actual_steps, "The function did not produce the expected number of steps.");
    }

    #[test]
    fn test_find_escape_steps_for_ghosts() {
        // Arrange
        let instructions = vec!['L', 'R'];
        let mut network = HashMap::new();
        network.insert("11A".to_string(), vec!["11B".to_string(), "XXX".to_string()]);
        network.insert("11B".to_string(), vec!["XXX".to_string(), "11Z".to_string()]);
        network.insert("11Z".to_string(), vec!["11B".to_string(), "XXX".to_string()]);
        network.insert("22A".to_string(), vec!["22B".to_string(), "XXX".to_string()]);
        network.insert("22B".to_string(), vec!["22C".to_string(), "22C".to_string()]);
        network.insert("22C".to_string(), vec!["22Z".to_string(), "22Z".to_string()]);
        network.insert("22Z".to_string(), vec!["22B".to_string(), "22B".to_string()]);
        network.insert("XXX".to_string(), vec!["XXX".to_string(), "XXX".to_string()]);

        let expected_steps = 6; // Expected number of steps as per the example

        // Act
        let actual_steps = find_escape_steps(instructions, network);

        // Assert
        assert_eq!(expected_steps, actual_steps, "The function did not produce the expected number of steps.");
    }
}


fn main() {
    let file_path = "puzzle_input.txt"; // Update with actual file path
    let (instructions, network) = read_puzzle_input(file_path).expect("Failed to read puzzle input");
    let expected_steps:usize = 2;
    let actual_steps = navigate_to_zzz(&instructions, &network);
    println!("The number of steps to reach ZZZ is: {}", actual_steps);

    let escape_steps = find_escape_steps(instructions, network);
    println!("The number of steps to escape is: {}", escape_steps);
}
