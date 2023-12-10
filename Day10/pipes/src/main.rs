use std::collections::{ HashSet};
use std::fs;
use geo::algorithm::contains::Contains;
use geo::{Polygon, Point};




fn main() {
    let grid = load_character_grid("input.txt");
    let path = build_path(&grid);
    let furthest_distance = (path.len() + (path.len() % 2)) / 2;
    println!("Furthest distance: {}", furthest_distance);
    let polygon = create_polygon(&path);
    let enclosed_tiles = count_enclosed_tiles(&grid, &polygon);
    println!("Enclosed tiles: {}", enclosed_tiles);

}

fn load_character_grid(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_connected(node: char, node_pos: usize, grid: &Vec<Vec<char>>) -> Vec<usize> {
    let width = grid[0].len();
    let height = grid.len();
    match node {
        '.' => vec![],
        '|' => {
            let mut connected = Vec::new();
            if node_pos >= width {
                connected.push(node_pos - width);
            }
            if node_pos + width < height * width {
                connected.push(node_pos + width);
            }
            connected
        },
        '-' => {
            let mut connected = Vec::new();
            if node_pos % width > 0 {
                connected.push(node_pos - 1);
            }
            if node_pos % width < width - 1 {
                connected.push(node_pos + 1);
            }
            connected
        },
        'L' => {
            let mut connected = Vec::new();
            if node_pos >= width {
                connected.push(node_pos - width);
            }
            if node_pos % width < width - 1 {
                connected.push(node_pos + 1);
            }
            connected
        },
        'J' => {
            let mut connected = Vec::new();
            if node_pos >= width {
                connected.push(node_pos - width);
            }
            if node_pos % width > 0 {
                connected.push(node_pos - 1);
            }
            connected
        },
        '7' => {
            let mut connected = Vec::new();
            if node_pos + width < height * width {
                connected.push(node_pos + width);
            }
            if node_pos % width > 0 {
                connected.push(node_pos - 1);
            }
            connected
        },
        'F' => {
            let mut connected = Vec::new();
            if node_pos + width < height * width {
                connected.push(node_pos + width);
            }
            if node_pos % width < width - 1 {
                connected.push(node_pos + 1);
            }
            connected
        },
        'S' => {
            let row = node_pos / width;
            let col = node_pos % width;

            let mut neighbors = Vec::new();

            // North
            if row > 0 {
                //north
                let north = (row - 1) * width + col;
                match grid[row-1][col] {
                    '|'|'F'|'7' => {
                        neighbors.push(north);
                    },
                    _ => {}
                }
            }

            // South
            if row < height - 1 {
                let south = (row + 1) * width + col;
                match grid[row+1][col] {
                    '|'|'J'|'L' => {
                        neighbors.push(south);
                    },
                    _ => {}
                }

            }
            // West
            if col > 0 {
               let west = row * width + (col - 1);
                match grid[row][col-1] {
                     '-'|'L'|'F' => {
                          neighbors.push(west);
                     },
                    _ => {}
                }
            }

            // East
            if col < width - 1 {
                let east = row * width + (col + 1);
                match grid[row][col+1] {
                    '-'|'J'|'7' => {
                        neighbors.push(east);
                    },
                    _ => {}
                }
            }
            assert_eq!(neighbors.len(), 2);
            neighbors
        },
        _ => vec![], // Handle unexpected characters
    }
}


fn build_path(grid: &Vec<Vec<char>>) -> Vec<Point<f64>> {
    let width = grid[0].len();
    let s_index = find_index_of_s(&grid).unwrap();
    let mut current_node_index = s_index;
    let mut vertices = Vec::new();

    loop {
        // Convert index to coordinates and add to vertices
        let current_point = index_to_point(current_node_index, width);
        println!("at: {:?}", current_point.x_y());
        vertices.push(current_point);

        let connected_nodes = find_connected(grid[current_node_index / width][current_node_index % width], current_node_index, &grid);
        println!("connected: {:?}", connected_nodes);
        let next_node_index = connected_nodes.iter().find(|&x| *x != current_node_index).unwrap();
        println!("next: {:?}", index_to_point(*next_node_index, width).x_y());
        if *next_node_index == s_index {
            println!("found S");
            break;
        }
        current_node_index = *next_node_index;
    }

    vertices
}


fn find_index_of_s(grid: &Vec<Vec<char>>) -> Option<usize> {
    let width = grid[0].len();

    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == 'S') {
            return Some(i * width + j);
        }
    }

    None // Return None if 'S' is not found in the grid
}

fn index_to_point(index: usize, width: usize) -> Point<f64> {
    let x = (index % width) as f64;
    let y = (index / width) as f64;
    Point::new(x, y)
}

fn create_polygon(vertices: &Vec<Point<f64>>) -> Polygon<f64> {
    Polygon::new(vertices.clone().into(), vec![])
}


fn count_enclosed_tiles(grid: &Vec<Vec<char>>, polygon: &Polygon<f64>) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != '.' && polygon.contains(&Point::new(x as f64, y as f64)) {
                count += 1;
            }
        }
    }

    count
}




fn print_grid(grid: &Vec<Vec<char>>) {
    println!();
    for row in grid {
        for &cell in row {
            print!("{}", cell);
        }
        println!(); // New line after each row
    }
    println!(); // Separator line for clarity
}


#[cfg(test)]
mod tests {
    use super::*;






    #[test]
    fn test_find_s_index() {
        let grid = load_character_grid("test_input.txt");
        let s_index = find_index_of_s(&grid).unwrap();
        assert_eq!(s_index, 10);
    }
#[test]
    fn test_count_enclosed_tiles() {
        let grid = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '-', '-', '-', '-', '-', '-', '7', '.'],
            vec!['.', '|', 'F', '-', '-', '-', '-', '-', '7', '|', '.'],
            vec!['.', '|', '|', '.', '.', '.', '.', '.', '|', '|', '.'],
            vec!['.', '|', '|', '.', '.', '.', '.', '.', '|', '|', '.'],
            vec!['.', '|', 'L', '-', '7', '.', 'F', '-', 'J', '|', '.'],
            vec!['.', '|', '.', '.', '|', '.', '|', '.', '.', '|', '.'],
            vec!['.', 'L', '-', '-', 'J', '.', 'L', '-', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

    let path = build_path(&grid);
    let polygon = create_polygon(&path);
    let enclosed_tiles = count_enclosed_tiles(&grid, &polygon);

        assert_eq!(enclosed_tiles, 4);
    }



    #[test]
    fn test_count_enclosed_tiles_example2() {
        let grid = load_character_grid("bigger_grid_input.txt");
        let path = build_path(&grid);
        let polygon = create_polygon(&path);
        let enclosed_tiles = count_enclosed_tiles(&grid, &polygon);
        assert_eq!(enclosed_tiles, 10);
    }

    #[test]
    fn test_count_enclosed_tiles_example() {
        let grid = load_character_grid("second_test_input.txt");
        let path = build_path(&grid);
        let polygon = create_polygon(&path);
        let enclosed_tiles = count_enclosed_tiles(&grid, &polygon);
        assert_eq!(enclosed_tiles, 8);
    }




    #[test]
    fn test_build_graph() {
        // Define the grid
        let grid = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];

        // Build the graph from the grid
        let path = build_path(&grid);

        // Nodes expected to be in the graph
        let expected_nodes: Vec<Point<f64>> = vec![
            index_to_point(6, grid[0].len()),  // 'S'
            index_to_point(7, grid[0].len()),  // '-'
            index_to_point(8, grid[0].len()),  // '7'
            index_to_point(11, grid[0].len()), // '|'
            index_to_point(13, grid[0].len()), // '|'
            index_to_point(16, grid[0].len()), // 'L'
            index_to_point(17, grid[0].len()), // '-'
            index_to_point(18, grid[0].len()), // 'J'
        ].into_iter().collect();



        // Check if all expected nodes are present
        for node in &path {

            assert!(expected_nodes.contains(&node), "Unexpected node in graph: {:?}", node.x_y());
        }

        // Check if the number of nodes matches the expected
        assert_eq!(path.len(), expected_nodes.len(), "Incorrect number of nodes in graph");

        // You can add additional checks here to validate the edges and their connections
    }


}