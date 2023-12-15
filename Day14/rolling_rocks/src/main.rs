use grid::Grid;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Function to load the grid from a file
fn load_grid_from_file(path: &Path) -> io::Result<Grid<char>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut grid_lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        grid_lines.push(line.chars().collect::<Vec<_>>());
    }

    // Assuming all lines are of equal length
    let rows = grid_lines.len();
    let cols = grid_lines.first().unwrap_or(&Vec::new()).len();

    let mut grid = Grid::new(cols, rows);

    for (y, line) in grid_lines.iter().enumerate() {
        for (x, &cell) in line.iter().enumerate() {
            grid.set(x, y, cell);
        }
    }

    Ok(grid)
}

fn main() {
    let path = Path::new("test.txt");
    match load_grid_from_file(path) {
        Ok(grid) => println!("{:?}", grid),
        Err(e) => println!("Error reading file: {}", e),
    }
}