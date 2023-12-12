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

fn find_connected(node: char, node_pos: (usize,usize), grid: &Vec<Vec<char>>) -> Vec<usize, usize> {
    let path = vec!([]);

    match node {
        'S' => {
            //look north
            if (node_pos.0) > 0{
                match grid[node_pos.0 -1][node_pos] {
                    '|' | '7' | 'F' => path.push((node_pos.0 -1, node_pos));
                    _ => {}
                }
            }
            //look south
            if (node_pos.0) < grid[0].len(){
                match grid[node_pos.0 +1][node_pos] {
                    '|' | 'J' | 'L' => path.push((node_pos.0 +1, node_pos));
                    _ => {}
                }
            }
            //look west
            if (node_pos.1) > 0{
                match grid[node_pos.1 -1][node_pos] {
                    '-' | '7' | 'F' => path.push((node_pos.0 -1, node_pos));
                    _ => {}
                }
            }
            // look east
            if (node_pos.1) < grid.len(){
                match grid[node_pos.1 +1][node_pos] {
                    '-' | 'J' | '7' => path.push((node_pos.0 +1, node_pos));
                    _ => {}
                }
            }
        }
        '|' => {
            //look north
            if (node_pos.0) > 0{
                match grid[node_pos.0 -1][node_pos] {
                    '|' | '7' | 'F' => path.push((node_pos.0 -1, node_pos));
                    _ => {}
                }
            }
            //look south
            if (node_pos.0) < grid[0].len(){
                match grid[node_pos.0 +1][node_pos] {
                    '|' | 'J' | 'L' => path.push((node_pos.0 +1, node_pos));
                    _ => {}
                }
            }
        }
        '-' => {
            //look west
            if (node_pos.1) > 0{
                match grid[node_pos.1 -1][node_pos] {
                    '-' | 'L' | 'F' => path.push((node_pos.0 -1, node_pos));
                    _ => {}
                }
            }
            // look east
            if (node_pos.1) < grid.len(){
                match grid[node_pos.1 +1][node_pos] {
                    '-' | 'J' | '7' => path.push((node_pos.1 +1, node_pos));
                    _ => {}
                }
            }
        }
        '7' => {
            //look north
            if (node_pos.0) > 0{
                match grid[node_pos.0 -1][node_pos] {
                    '|' | '7' | 'F' => path.push((node_pos.0 -1, node_pos));
                    _ => {}
                }
            }
            // look east
            if (node_pos.1) < grid.len(){
                match grid[node_pos.1 +1][node_pos] {
                    '-' | 'J' | '7' => path.push((node_pos.0 +1, node_pos));
                    _ => {}
                }
            }
        }
        'F' => {
            //look south
            if (node_pos.0) < grid[0].len(){
                match grid[node_pos.0 +1][node_pos] {
                    '|' | 'J' | 'L' => path.push((node_pos.1 +1, node_pos));
                    _ => {}
                }
            }
            // look east
            if (node_pos.1) < grid.len(){
                match grid[node_pos.1 +1][node_pos] {
                    '-' | 'J' | '7' => path.push((node_pos.0 +1, node_pos));
                    _ => {}
                }
            }
        }
        _ => {}
    }

}


fn build_path(grid: &Vec<Vec<char>>) -> Vec<usize, usize> {

}


fn find_index_of_s(grid: &Vec<Vec<char>>) -> Option<usize, usize> {
    let width = grid.len();

    for (i, col) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == 'S') {
            return Some(i * width +j);
        }
    }

    None // Return None if 'S' is not found in the grid
}



fn create_polygon(vertices: &Vec<Vec<usize,usize>>) -> Polygon<f64> {
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