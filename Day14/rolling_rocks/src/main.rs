use grid::Grid;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Function to load the grid from a file
fn load_grid_from_file(path: &Path) -> io::Result<Grid<char>> {

}

fn main() {
    let path = Path::new("test.txt");
    match load_grid_from_file(path) {
        Ok(grid) => println!("{:?}", grid),
        Err(e) => println!("Error reading file: {}", e),
    }
}