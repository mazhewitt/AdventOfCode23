use std::collections::{ HashSet};
use std::fs;
use geo::algorithm::contains::Contains;
use geo::{Polygon, Point};




fn main() {
    let grid = load_character_grid("input.txt");
    let path = build_path(&grid);
    let furthest_distance = (path.len() + (path.len() % 2)) / 2;
    println!("Furthest distance: {}", furthest_distance);


}

fn load_character_grid(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_connected(node_pos: (usize,usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut path:Vec <(usize, usize)> = Vec::new();
    let (r, c) = node_pos;
    let node = grid[r][c];
    match node {
        'S' => {
            //look north
            if r > 0 {
                match grid[r -1][c] {
                    '|' | '7' | 'F' | 'S' => path.push((r -1, c)),
                    _ => {}
                }
            }
            //look south
            if r < grid[0].len(){
                match grid[r +1][c] {
                    '|' | 'J' | 'L'| 'S' => path.push((r +1, c)),
                    _ => {}
                }
            }
            //look west
            if c > 0{
                match grid[r][c-1] {
                    '-' | '7' | 'F'| 'S' => path.push((r, c -1)),
                    _ => {}
                }
            }
            // look east
            if (c) < grid.len(){
                match grid[r][c+1] {
                    '-' | 'J' | '7'| 'S' => path.push((r, c+1)),
                    _ => {}
                }
            }
        }
        '|' => {
            //look north
            if (r) > 0{
                match grid[r -1][c] {
                    '|' | '7' | 'F'| 'S' => path.push((r -1, c)),
                    _ => {}
                }
            }
            //look south
            if (r) < grid[0].len(){
                match grid[r +1][c] {
                    '|' | 'J' | 'L'| 'S' => path.push((r +1, c)),
                    _ => {}
                }
            }
        }
        '-' => {
            //look west
            if (c) > 0{
                match grid[r][c-1] {
                    '-' | 'L' | 'F'| 'S' => path.push((r, c -1)),
                    _ => {}
                }
            }
            // look east
            if (c) < grid.len(){
                match grid[r][c+1] {
                    '-' | 'J' | '7'| 'S' => path.push((r, c +1)),
                    _ => {}
                }
            }
        }
        '7' => {
            //look south
            if (r) < grid[0].len(){
                match grid[r +1][c] {
                    '|' | 'J' | 'L'| 'S' => path.push((r +1, c)),
                    _ => {}
                }
            }
            //look west
            if (c) > 0{
                match grid[r][c-1] {
                    '-' | 'L' | 'F'| 'S' => path.push((r, c -1)),
                    _ => {}
                }
            }
        }
        'F' => {
            //look south
            if (r) < grid[0].len(){
                match grid[r +1][c] {
                    '|' | 'J' | 'L'| 'S' => path.push((r +1, c)),
                    _ => {}
                }
            }
            // look east
            if (c) < grid.len(){
                match grid[r][c+1] {
                    '-' | 'J' | '7'| 'S' => path.push((r, c +1)),
                    _ => {}
                }
            }
        }
        'L' => {
            //look north
            if (r) > 0{
                match grid[r -1][c] {
                    '|' | '7' | 'F'| 'S' => path.push((r -1, c)),
                    _ => {}
                }
            }
            // look east
            if (c) < grid.len(){
                match grid[r][c+1] {
                    '-' | 'J' | '7'| 'S' => path.push((r, c +1)),
                    _ => {}
                }
            }
        }
        'J' => {
            //look north
            if (r) > 0{
                match grid[r -1][c] {
                    '|' | '7' | 'F'| 'S' => path.push((r -1, c)),
                    _ => {}
                }
            }
            //look west
            if (c) > 0{
                match grid[r][c-1] {
                    '-' | 'L' | 'F'| 'S' => path.push((r, c -1)),
                    _ => {}
                }
            }
        }
        _ => {}
    }
    //println!("Node: {:?}, {} has {} connections", node_pos, grid[r][c],  path.len());
    assert_eq!(path.len(), 2);
    path
}


fn build_path(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let start = find_start_position(grid).expect("Start position not found");
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    dfs(start, &mut visited, &mut path, grid);
    path
}

fn dfs(
    pos: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    path: &mut Vec<(usize, usize)>,
    grid: &Vec<Vec<char>>
) {
    if visited.contains(&pos) {
        return;
    }

    visited.insert(pos);
    path.push(pos);

    let connected_nodes = find_connected(pos, grid);
    for next_pos in connected_nodes {
        dfs(next_pos, visited, path, grid);
    }
}

fn find_start_position(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return Some((i, j));
            }
        }
    }
    None
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