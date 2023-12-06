use std::fs;
use std::fs::File;

fn main() {
    println!("Hello, world!");
}

fn load_character_grid(filename: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut file = File::open(filename).unwrap();
    let mut contents = fs::read_to_string(filename).unwrap();
    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}


//Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_character_grid() {
        // a two dimensional array of characters


        let grid = load_character_grid("test_file.txt");
        assert_eq!(grid[0].len(), 10);
        assert_eq!(grid.len(), 10);
    }
}