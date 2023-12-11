use std::collections::HashSet;
use std::fs;

fn main() {
    let grid = load_character_grid("galaxy_data.txt");

    let dot_cols:HashSet<usize> = find_dot_columns(&grid);
    let dot_rows:HashSet<usize> = find_dot_rows(&grid);
    let galaxies:HashSet<(usize, usize)> = load_galaxy_positions(&grid);
    // sum all the distances between all the galaxies
    let total_distance = calculate_total_distance(&dot_cols, &dot_rows, &galaxies,2);
    println!("Total distance between all galaxies: {}", total_distance);
    let total_distance = calculate_total_distance(&dot_cols, &dot_rows, &galaxies,1000000);
    println!("Total distance between all galaxies: {}", total_distance);
}

fn calculate_total_distance(dot_cols: &HashSet<usize>, dot_rows: &HashSet<usize>, galaxies: &HashSet<(usize, usize)>, expansion_factor:usize) -> usize {
    // calculate total distance only once between each pair of galaxies
    let mut total_distance = 0;
    let mut galaxy_pairs = HashSet::new();
    for g0 in galaxies {
        for g1 in galaxies {
            if g0 != g1 && !galaxy_pairs.contains(&(g1, g0)) {
                let distance = find_distance_between_galaxies(*g0, *g1, dot_rows, dot_cols, expansion_factor);
                total_distance += distance;
                galaxy_pairs.insert((g0, g1));
            }
        }
    }
    total_distance
}

fn load_galaxy_positions(grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    grid.iter().enumerate().flat_map(|(row_index, row)| {
        row.iter().enumerate().filter_map(move |(col_index, &c)| {
            if c == '#' {
                Some((row_index, col_index ))
            } else {
                None
            }
        })
    }).collect()
}

fn load_character_grid(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_dot_rows(p0: &Vec<Vec<char>>) -> HashSet<usize> {
    // loop though the rows, if all the characters are '.' then add the row index to the set
    p0.iter().enumerate().filter_map(|(row_index, row)| {
        if row.iter().all(|&c| c == '.') {
            Some(row_index)
        } else {
            None
        }
    }).collect()
}

fn find_dot_columns(p0: &Vec<Vec<char>>) -> HashSet<usize> {
    // return a set of column indexes that are all '.'
    let mut dot_cols = HashSet::new();
    for col_index in 0..p0[0].len() {
        let mut all_dots = true;
        for row in p0 {
            if row[col_index] != '.' {
                all_dots = false;
                break;
            }
        }
        if all_dots {
            dot_cols.insert(col_index);
        }
    }
    dot_cols
}

fn find_distance_between_galaxies(g0: (usize, usize), g1: (usize, usize), dot_rows: &HashSet<usize>, dot_cols: &HashSet<usize>, expansion_factor:usize) -> usize {
    let (r0, c0) = g0;
    let (r1, c1) = g1;

    let dr = std::cmp::max(r0, r1) - std::cmp::min(r0, r1);
    let dc = std::cmp::max(c0, c1) - std::cmp::min(c0, c1);

    let dot_cols_between = (std::cmp::min(c0, c1)..std::cmp::max(c0, c1)).filter(|&col| dot_cols.contains(&col)).count();
    let dot_rows_between = (std::cmp::min(r0, r1)..std::cmp::max(r0, r1)).filter(|&row| dot_rows.contains(&row)).count();

    dr + dc +  dot_cols_between*expansion_factor - dot_cols_between +  dot_rows_between*expansion_factor - dot_rows_between
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_load_character_grid() {
        let grid = load_character_grid("test_data.txt");
        assert_eq!(grid.len(), 10);
        assert_eq!(grid[0].len(), 10);
    }

    #[test]
    fn test_load_symbol_set() {
        let grid = load_character_grid("test_data.txt");
        let symbol_set: HashSet<(usize,usize)> = load_galaxy_positions(&grid);
        assert_eq!(symbol_set.len(), 9);
        assert!(symbol_set.contains(&(0, 3)));
        assert!(symbol_set.contains(&(1, 7)));
        assert!(symbol_set.contains(&(9, 0)));
    }

    #[test]
    fn test_find_dot_rows() {
        let grid = load_character_grid("test_data.txt");

        let dot_rows:HashSet<usize> = find_dot_rows(&grid);
        assert_eq!(dot_rows.len(), 2);
        assert!(dot_rows.contains(&3));
        assert!(dot_rows.contains(&7));
    }
    #[test]
    fn test_find_dot_columns() {
        let grid = load_character_grid("test_data.txt");

        let dot_cols:HashSet<usize> = find_dot_columns(&grid);
        assert_eq!(dot_cols.len(), 3);
        assert!(dot_cols.contains(&2));
        assert!(dot_cols.contains(&5));
        assert!(dot_cols.contains(&8));
    }

    #[test]
    fn test_distance_same_row() {
        let g0 = (2, 1);
        let g1 = (2, 4);
        let dot_rows: HashSet<usize> = [0, 3].iter().cloned().collect();
        let dot_cols: HashSet<usize> = [0, 5].iter().cloned().collect();
        let expected = 3;
        assert_eq!(find_distance_between_galaxies(g0, g1, &dot_rows, &dot_cols, 2), expected);
    }

    #[test]
    fn test_distance_same_column_expanded_rows() {
        let g0 = (1, 3);
        let g1 = (4, 3);
        let dot_rows: HashSet<usize> = [2].iter().cloned().collect();
        let dot_cols: HashSet<usize> = [0, 5].iter().cloned().collect();
        let expected = 4;
        assert_eq!(find_distance_between_galaxies(g0, g1, &dot_rows, &dot_cols, 2), expected);
    }

    #[test]
    fn test_distance_diagonal_expanded() {
        let g0 = (0, 0);
        let g1 = (4, 4);
        let dot_rows: HashSet<usize> = [2].iter().cloned().collect();
        let dot_cols: HashSet<usize> = [2].iter().cloned().collect();
        let expected = 10;
        assert_eq!(find_distance_between_galaxies(g0, g1, &dot_rows, &dot_cols,2 ), expected);
    }

    #[test]
    fn test_distance_multiple_expanded_rows_cols() {
        let g0 = (0, 0);
        let g1 = (5, 5);
        let dot_rows: HashSet<usize> = [1, 3].iter().cloned().collect();
        let dot_cols: HashSet<usize> = [1, 3].iter().cloned().collect();
        let expected = 14;
        assert_eq!(find_distance_between_galaxies(g0, g1, &dot_rows, &dot_cols, 2), expected);
    }


    #[test]
    fn test_find_the_distance_between_two_galaxies() {
        let galaxy1 = (0, 3);
        let galaxy2 = (1, 7);
        let galaxy3 = (9, 0);
        let grid = load_character_grid("test_data.txt");

        let dot_cols:HashSet<usize> = find_dot_columns(&grid);
        let dot_rows:HashSet<usize> = find_dot_rows(&grid);
        let distance = find_distance_between_galaxies(galaxy1, galaxy2, &dot_rows, &dot_cols,2);

        assert_eq!(distance, 6);
        let distance2 = find_distance_between_galaxies(galaxy1, galaxy3, &dot_rows, &dot_cols,2 );
        assert_eq!(distance2, 15);
        let total_distance = calculate_total_distance(&dot_cols, &dot_rows, &load_galaxy_positions(&grid),2);
        assert_eq!(total_distance, 374)
    }


}