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

    let score = calculate_weight(grid);

    println!("score: {}", score);

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

    for _ in 0..4 {
        new_grid = rotate_90_degrees_clockwise(&new_grid);
        new_grid = tilt_grid_in_place(&new_grid);
    }

    new_grid
}

fn rotate_90_degrees_clockwise(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = matrix.len();
    let mut rotated = vec![vec![' '; n]; n];

    for i in 0..n {
        for j in 0..n {
            rotated[j][n - i - 1] = matrix[i][j];
        }
    }

    rotated
}

fn tilt_grid_in_place(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter().map(|row| {
        let mut parts: Vec<Vec<char>> = row.split(|&ch| ch == '#')
            .map(|part| {
                let mut chars: Vec<char> = part.to_vec();
                chars.sort_by(|a, b| b.cmp(a));
                chars
            })
            .collect();

        if parts.len() > 1 {
            parts.insert(1, vec!['#']);
        }

        parts.concat()
    }).collect()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

// ... rest of your code ...


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::ops::Index;
    use super::*;
    #[test]
    fn test_load_character_grid() {
        let grid = transpose(&load_character_grid("test.txt"));

        let score = calculate_weight(grid);

        assert_eq!(score, 136);
    }

    #[test]
    fn weight_after_many_rotations() {
        let mut grid = transpose(&load_character_grid("test.txt"));
        let mut seen_grids: HashSet<Vec<Vec<char>>> = HashSet::new();
        let mut score = 0;
        let mut cycle_count = 0;
        let mut all_grids: Vec<Vec<Vec<char>>> = Vec::new();
        loop{
            cycle_count += 1;
            let new_gird = tilt_cycle(&grid);
            if seen_grids.contains(&new_gird) {
                let first = all_grids.iter().position(|x| *x == new_gird).unwrap();
                let score_grid = &all_grids[(1000000000 - first) % (cycle_count - first) + first];
                score = calculate_score(10,score_grid);
                break;
            }
            seen_grids.insert(new_gird.clone());
            all_grids.push(new_gird.clone());
            grid = new_gird;
        }

        assert_eq!(score, 64);
    }

    #[test]
    fn test_1_cycle(){
        let grid = transpose(&load_character_grid("test.txt"));
        let tilted_grid = tilt_cycle(&grid);
        let transposed_grid = transpose(&tilted_grid);
        print_grid(&transposed_grid);
        let two_cycle = tilt_cycle(&tilted_grid);
        let transposed_2_grid = transpose(&two_cycle);
        println!();
        println!("2 cycle");
        print_grid(&transposed_2_grid);
        let three_cycle = tilt_cycle(&two_cycle);
        let transposed_3_grid = transpose(&three_cycle);
        println!();
        println!("3 cycle");
        print_grid(&transposed_3_grid);
        calculate_score(10,  &grid);
    }
    #[cfg(test)]
    mod tests {
        use super::*;


    }


}