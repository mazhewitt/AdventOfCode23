use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let instructions = load_instructions("input.txt", false);
    let path = track_path(&instructions);
    let area = caclulate_total_area(&path);
    println!("Total area: {}", area);
    let part2_instructions = load_instructions("input.txt", true);
    let part2_path = track_path(&part2_instructions);
    let part2_area = caclulate_total_area(&part2_path);
    println!("Total area: {}", part2_area);
}

fn load_instructions(file: &str, part2:bool) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if part2 {
            let instruction = parse_instruction_part2(&line).expect("Failed to parse instruction");
            instructions.push(instruction);
        }
        else {
            let instruction = parse_instruction(&line).expect("Failed to parse instruction");
            instructions.push(instruction);
        }
    }
    instructions
}


#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    distance: u32,
    color: String,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// `parse_instruction` function to be implemented.
fn parse_instruction(input: &str) -> Result<Instruction, String> {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    if parts.len() < 2 {
        return Err("Input string is too short".to_string());
    }

    let direction = match parts[0] {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => return Err(format!("Invalid direction: {}", parts[0])),
    };

    let distance = parts[1].parse::<u32>()
        .map_err(|_| format!("Invalid distance: {}", parts[1]))?;

    let color = parts.get(2).map(|c| c.trim_matches(|p| p == '(' || p == ')').to_string()).unwrap();

    Ok(Instruction { direction, distance, color })
}

fn track_path(instructions: &[Instruction]) -> Vec<(isize, isize)> {
    let total_distance: usize = instructions.iter().map(|inst| inst.distance as usize).sum();
    let mut path = Vec::with_capacity(total_distance);
    let mut current_position = (0, 0);
    path.push(current_position);
    for instruction in instructions {
        let (row_step, col_step) = match instruction.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        for _ in 0..instruction.distance {
            current_position.0 += row_step;
            current_position.1 += col_step;
            path.push(current_position);
        }
    }
    path
}

// The `apply_shoelace_formula`
fn apply_shoelace_formula(path: &[(isize, isize)]) -> isize {
    let n = path.len();
    if n < 3 {
        // A polygon must have at least 3 vertices
        return 0;
    }

    let mut area = 0;
    for i in 0..n {
        let (_x, y) = path[i];
        let x_prev = path[(i + n - 1) % n].0; // x-coordinate of the previous vertex
        let x_next = path[(i + 1) % n].0;     // x-coordinate of the next vertex

        area += y * (x_prev - x_next);
    }

    (area / 2).abs()
}

// The `apply_picks_theorem`
fn caclulate_total_area(path: &[(isize, isize)]) -> isize {
    let area = apply_shoelace_formula(path);
    let boundary_points = count_boundary_points(path);
    (area - boundary_points / 2) + boundary_points +1
}

// Count unique boundary points, considering the path is closed.
fn count_boundary_points(path: &[(isize, isize)]) -> isize {
    let mut unique_points = path.to_vec();
    unique_points.pop(); // Remove the duplicate point at the end
    unique_points.sort();
    unique_points.dedup();
    unique_points.len() as isize
}

fn parse_instruction_part2(input: &str) -> Result<Instruction, String> {
    let color_code = input.split_whitespace().last().ok_or("Invalid input format")?.trim_matches(|c| c == '(' || c == ')');

    if color_code.len() != 7 || !color_code.starts_with('#') {
        return Err("Invalid color code format".to_string());
    }

    let distance_str = &color_code[1..6];
    let direction_code = color_code.chars().last().unwrap();

    let distance = u32::from_str_radix(distance_str, 16)
        .map_err(|_| "Failed to parse distance from hexadecimal".to_string())?;

    let direction = match direction_code {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => return Err("Invalid direction code".to_string()),
    };

    Ok(Instruction {
        direction,
        distance,
        color: color_code.to_string(),
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let input = "R 6 (#70c710)";
        let expected = Instruction {
            direction: Direction::Right,
            distance: 6,
            color: "#70c710".to_string()
        };
        assert_eq!(parse_instruction(input).unwrap(), expected);
    }

    #[test]
    fn test_track_path() {
        let instructions = load_instructions("test.txt");

        let expected_path = vec![
            (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6),
            (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (5, 5), (5, 4),
            (6, 4), (7, 4), (7, 5), (7, 6), (8, 6), (9, 6), (9, 5),
            (9, 4), (9, 3), (9, 2), (9, 1), (8, 1), (7, 1), (7, 0),
            (6, 0), (5, 0), (5, 1), (5, 2), (4, 2), (3, 2), (2, 2),
            (2, 1), (2, 0), (1, 0), (0, 0),
        ];
        let path:Vec<(isize, isize)> = track_path(&instructions);
        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_shoelace_formula_rectangle() {
        let path = vec![
            (0, 0), (0, 4), (3, 4), (3, 0), (0, 0) // Closing the path
        ];

        let expected_area = 12; // Area of a 4x3 rectangle.

        assert_eq!(apply_shoelace_formula(&path), expected_area);
    }




    #[test]
    fn test_picks_theorem_with_expanded_path() {
        // Example path representing a 4x3 rectangle with all boundary points included
        let path = vec![
            (0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
            (1, 4), (2, 4), (3, 4),
            (3, 3), (3, 2), (3, 1), (3, 0),
            (2, 0), (1, 0), (0, 0) // Closing the path
        ];

        assert_eq!(caclulate_total_area(&path), 20);
    }


    #[test]
    fn test_example_formula() {

        let path = vec![
            (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6),
            (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (5, 5), (5, 4),
            (6, 4), (7, 4), (7, 5), (7, 6), (8, 6), (9, 6), (9, 5),
            (9, 4), (9, 3), (9, 2), (9, 1), (8, 1), (7, 1), (7, 0),
            (6, 0), (5, 0), (5, 1), (5, 2), (4, 2), (3, 2), (2, 2),
            (2, 1), (2, 0), (1, 0), (0, 0),
        ];
        assert_eq!(caclulate_total_area(&path), 62);

    }

    #[test]
    fn parse_part_2_input() {
        let input1 = "R 6 (#70c710)".to_string();
        let instruction1 = parse_instruction_part2(&input1).unwrap();
        let expected1 = Instruction {
            direction: Direction::Right,
            distance: 461937,
            color: "#70c710".to_string()
        };
        assert_eq!(instruction1, expected1);

        let input2 = "D 5 (#0dc571)".to_string();
        let instruction2 = parse_instruction_part2(&input1).unwrap();
        let expected2 = Instruction {
            direction: Direction::Down,
            distance: 56407,
            color: "#0dc571".to_string()
        };
        assert_eq!(instruction2, expected2);
    }


}

