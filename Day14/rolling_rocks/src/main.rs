use std::collections::HashMap;
use std::fs;


// Function to load the grid from a file
fn load_character_grid(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn main() {
    let grid = transpose(&load_character_grid("input.txt"));
    let part2_grid = grid.clone();
    let score = calculate_weight(grid);

    println!("score: {}", score);

    let part2_score = calculate_score_after_billion_rotations(part2_grid);
    println!("part 2 score: {}", part2_score-1);

}
fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    let mut transposed = vec![vec![' '; num_rows]; num_cols];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            transposed[j][i] = item;
        }
    }

    transposed
}

fn calculate_weight(grid: Vec<Vec<char>>) -> usize {
    let size = grid.len();
    let tilted_grid = tilt_grid_in_place(&grid);
    print_grid(&tilted_grid);
    calculate_score(size, &tilted_grid)
}

fn calculate_score(size: usize, tilted_grid: &Vec<Vec<char>>) -> usize {
    let mut score: usize = 0;
    for column in tilted_grid {
        for (r, &char) in column.iter().enumerate() {
            if char == 'O' {
                score += size - (r);
            }
        }
    }
    score
}

fn tilt_cycle(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = grid.clone();

    for _ in 1..=4 {
        new_grid = tilt_grid_in_place(&new_grid);
        new_grid = rotate_90_degrees_counterclockwise(&new_grid);
    }
    new_grid
}

fn rotate_90_degrees_clockwise(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    println!("rotating");
    let n = matrix.len();
    let mut rotated = vec![vec![' '; n]; n];

    for i in 0..n {
        for j in 0..n {
            rotated[j][n - i - 1] = matrix[i][j];
        }
    }

    rotated
}

fn rotate_90_degrees_counterclockwise(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = matrix.len();
    let mut rotated = vec![vec![' '; n]; n];

    for i in 0..n {
        for j in 0..n {
            rotated[n - j - 1][i] = matrix[i][j];
        }
    }

    rotated
}

fn tilt_grid_in_place(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let tilted:Vec<Vec<char>> = grid.iter().map(|row| {
        let mut parts: Vec<Vec<char>> = row.split(|&ch| ch == '#')
            .map(|part| {
                let mut chars: Vec<char> = part.to_vec();
                chars.sort_by(|a, b| b.cmp(a));
                chars
            })
            .collect();

        // join the parts back together with a # in between
        let mut new_row = Vec::new();
        for part in parts.drain(..) {
            new_row.extend(part);
            new_row.push('#');
        }
        new_row.pop(); // remove the last #
        new_row
    }).collect();

    assert_eq!(tilted.len(), grid.len());
    assert_eq!(tilted[0].len(), grid[0].len());
    tilted
}

fn print_grid(grid: &Vec<Vec<char>>) {
    let transposed_grid = transpose(&grid);
    for row in transposed_grid {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn calculate_score_after_billion_rotations(mut grid: Vec<Vec<char>>) -> usize {
    let mut seen_grids: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let score;
    let mut cycle_count = 0;
    let target = 1_000_000_000;

    loop {
        cycle_count += 1;
        let new_grid = tilt_cycle(&grid);

        if let Some(&first_seen_cycle) = seen_grids.get(&new_grid) {
            // Cycle detected
            let cycle_length = cycle_count - first_seen_cycle;
            let remaining_cycles = (target - cycle_count) % cycle_length;
            // Skip cycles
            for _ in 0..remaining_cycles {
                cycle_count += 1;
                grid = tilt_cycle(&grid);
            }
            score = calculate_score(grid.len(), &grid);
            break;
        } else {
            // Store grid state and cycle count
            seen_grids.insert(new_grid.clone(), cycle_count);
            grid = new_grid;
        }
    }
    score
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_character_grid() {
        let grid = transpose(&load_character_grid("test.txt"));

        let score = calculate_weight(grid);

        assert_eq!(score, 136);
    }

    #[test]
    fn weight_after_many_rotations() {

        let  grid = transpose(&load_character_grid("test.txt"));
        let score = calculate_score_after_billion_rotations(grid);
        assert_eq!(score, 64);



    }

    #[test]
    fn test_1_cycle(){
        let grid = transpose(&load_character_grid("test.txt"));
        let tilted_grid = tilt_cycle(&grid);
        print_grid(&tilted_grid);
        let two_cycle = tilt_cycle(&tilted_grid);
        println!("2 cycle");
        print_grid(&two_cycle);
        let three_cycle = tilt_cycle(&two_cycle);
        println!("3 cycle");
        print_grid(&three_cycle);
        calculate_score(10,  &grid);
    }
    #[test]
    fn test_rotate_90_degrees_clockwise() {
        let matrix = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];

        let expected_rotated_matrix = vec![
            vec!['7', '4', '1'],
            vec!['8', '5', '2'],
            vec!['9', '6', '3'],
        ];

        let rotated_matrix = rotate_90_degrees_clockwise(&matrix);

        assert_eq!(rotated_matrix, expected_rotated_matrix);
    }

}