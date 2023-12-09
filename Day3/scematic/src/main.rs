use std::collections::HashSet;
use std::fs;
use std::fs::File;
use regex::Regex;

fn main() {
    let grid = load_character_grid("input_file.txt");
    let sum = calculate_sum_of_numbers_with_adjacent_symbols(&grid);
    println!("Sum: {}", sum);

    let gear_ratios = calculate_gear_ratios(&grid);
    println!("Gear Ratios: {}", gear_ratios);
}

fn load_character_grid(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
fn load_symbol_set(grid: &Vec<Vec<char>>, is_symbol: fn(char)->bool) -> HashSet<(usize, usize)> {
    grid.iter().enumerate().flat_map(|(row_index, row)| {

        row.iter().enumerate().filter_map(move |(col_index, &c)| {
            if is_symbol(c) {
                Some((col_index, row_index))
            } else {
                None
            }
        })
    }).collect()
}

fn calculate_gear_ratios(grid: &Vec<Vec<char>>) -> usize {
    let numbers = find_numbers(grid);
    let star_set = load_symbol_set(grid, |c| c == '*');

    star_set.iter().filter_map(|&star| {
        let adjacent_numbers: Vec<_> = numbers.iter()
            .filter(|n| n.adjacent_points.contains(&star))
            .map(|n| n.number)
            .collect();

        if adjacent_numbers.len() == 2 {
            Some(adjacent_numbers[0] * adjacent_numbers[1])
        } else {
            None
        }
    }).sum()
}

#[derive(Debug)]
struct NumberLocation {
    point: (usize, usize), // start of the number
    number: usize,
    adjacent_points: HashSet<(usize, usize)>,
}

impl NumberLocation {
    fn new(point: (usize, usize), number: usize) -> NumberLocation {
        NumberLocation {
            point,
            number,
            adjacent_points: HashSet::new(),
        }
    }
}

fn find_numbers(grid: &[Vec<char>]) -> Vec<NumberLocation> {
    let mut numbers = Vec::new();
    let grid_width = grid.first().map_or(0, |row| row.len());
    let grid_height = grid.len();
    let re = Regex::new(r"\d+").unwrap();

    for (row_index, row) in grid.iter().enumerate() {
        let row_str: String = row.iter().collect();
        for mat in re.find_iter(&row_str) {
            let number_start = (row_index, mat.start());
            let number: usize = mat.as_str().parse().unwrap();
            numbers.push(NumberLocation {
                point: number_start,
                number,
                adjacent_points: adjacent_points_to_number(&mat.as_str().to_string(), number_start, grid_width, grid_height),
            });
        }
    }

    numbers
}


fn adjacent_points_to_number(number: &String, point: (usize, usize), grid_width: usize, grid_height: usize) -> HashSet<(usize, usize)> {
    let num_length = number.to_string().len();

    (-1i32..=1)
        .flat_map(move |d_row| {
            (-1i32..=1).flat_map(move |d_col| {
                (0..num_length).filter_map(move |offset| {
                    let adj_row = point.0 as i32 + d_row;
                    let adj_col = (point.1 + offset) as i32 + d_col;

                    if adj_row >= 0 && adj_row < grid_height as i32 && adj_col >= 0 && adj_col < grid_width as i32
                        && !(d_row == 0 && d_col == 0 && offset == 0) {
                        Some(( adj_col as usize, adj_row as usize))
                    } else {
                        None
                    }
                })
            })
        })
        .collect()
}

fn calculate_sum_of_numbers_with_adjacent_symbols(grid: &Vec<Vec<char>>) -> usize {
    let numbers = find_numbers(grid);
    let symbol_set = load_symbol_set(grid, |c| !c.is_numeric() && c != '.');

    numbers.into_iter().filter(|n| {
        n.adjacent_points.iter().any(|point| symbol_set.contains(point))
    }).map(|n| n.number).sum()
}


//Tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_load_character_grid() {
        // a two dimensional array of characters

        let grid = load_character_grid("test_file.txt");
        assert_eq!(grid[0].len(), 10);
        assert_eq!(grid.len(), 10);
    }

    #[test]
    fn test_load_symbol_set() {
        let points = vec![(3, 1), (6, 3), (3, 4), (5, 5), (3, 8), (5, 8)];

        // Creating a HashSet from the vector of points
        let symbol_points: HashSet<(usize, usize)> = HashSet::from_iter(points.into_iter());
        let character_grid = load_character_grid("test_file.txt");
        let loaded_points = load_symbol_set(&character_grid, |c| !c.is_numeric() && c != '.');
        assert_eq!(symbol_points, loaded_points);

    }


    #[test]
    fn test_calculate_sum_of_numbers_with_adjacent_symbols(){
        let grid = load_character_grid("test_file.txt");
        let sum = calculate_sum_of_numbers_with_adjacent_symbols(&grid);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_calc_gear_ratios(){
        let grid = load_character_grid("test_file.txt");
        let gear_ratios = calculate_gear_ratios(&grid);
        assert_eq!(gear_ratios, 467835);
    }


}