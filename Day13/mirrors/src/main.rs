use std::fs;

fn main() {
    let character_grids = load_character_grids("input_data.txt");
    let mut total_reflection_score = 0;
    for grid in &character_grids {
        total_reflection_score += caclulate_symmerty_score(&grid, false);
    }
    println!("Part 1: {}", total_reflection_score);

    total_reflection_score = 0;
    for grid in &character_grids {
        total_reflection_score += caclulate_symmerty_score(&grid, true);
    }
    println!("Part 2: {}", total_reflection_score);
}

fn load_character_grids(filename: &str) -> Vec<Vec<Vec<char>>> {
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    content.trim()
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>()
}

fn caclulate_symmerty_score(grid: &Vec<Vec<char>>, part2:bool) -> usize {


    let asymmetry_score_r = calculate_score_g(grid, part2);
    let grid_t = transpose(grid);
    let asymmetry_score_c = calculate_score_g(&grid_t, part2);


    asymmetry_score_r + (asymmetry_score_c * 100)

}

fn calculate_score_g(grid: &Vec<Vec<char>>, part2: bool) -> usize {
    let mut total_reflection_score = 0;
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    // Check for vertical symmetry
    for col in 0..num_cols - 1 {
        let mut asymmetry_score = 0;

        for offset in 0..num_cols {
            let left_col_index = col as isize - offset as isize;
            let right_col_index = col as isize + 1 + offset as isize;

            if left_col_index >= 0 && right_col_index < num_cols as isize {
                for row in 0..num_rows {
                    if grid[row][left_col_index as usize] != grid[row][right_col_index as usize] {
                        asymmetry_score+= 1;
                    }
                }
            }

        }
        if (asymmetry_score == 0 && !part2) || (asymmetry_score == 1 && part2) {
            total_reflection_score += col + 1;
        }
    }
    total_reflection_score
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let test_springs = load_character_grids("test_data.txt");
        assert_eq!(test_springs.len(), 2);
        assert_eq!(test_springs[0].len(), 7);
        assert_eq!(test_springs[0][0].len(), 9);
    }

    #[test]
    fn test_first_grid() {
        let test_grid = load_character_grids("test_data.txt");
        let g0 = caclulate_symmerty_score(&test_grid[0], false);
        let g1 = caclulate_symmerty_score(&test_grid[1], false);
        assert_eq!(g0+g1, 405);
    }

}