use std::collections::{ HashSet};
use std::fs;
use petgraph::graph::{UnGraph};



fn main() {
    let grid = load_character_grid("input.txt");
    let graph = build_path(&grid);
    let furthest_distance = (graph.node_count() + (graph.node_count() % 2)) / 2;
    println!("Furthest distance: {}", furthest_distance);
    let enclosed_tiles = count_enclosed_tiles(&grid, &graph);
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


fn build_path(grid: &Vec<Vec<char>>) -> UnGraph<usize, ()> {
    let width = grid[0].len();
    let mut graph = UnGraph::new_undirected();
    let s_index = find_index_of_s(&grid).unwrap();
    // add s to graph
    let mut current_node = graph.add_node(s_index);
    let nodes_connected_to_s = find_connected('S', s_index, &grid);
    let mut current_node_index = nodes_connected_to_s[0];
    let mut last_node_index = s_index;
    loop {
        // we are at the beginning
        if current_node_index == s_index {
            break;
        }
        let last_node = current_node;
        current_node = graph.add_node(current_node_index);
        graph.add_edge(last_node, current_node, ());
        let connected_nodes = find_connected(grid[current_node_index / width][current_node_index % width], current_node_index, &grid);
        // find the connected node that isn't last_node
        let next_node_index = connected_nodes.iter().find(|&x| *x != last_node_index).unwrap();
        last_node_index = current_node_index;
        current_node_index = *next_node_index;

    }

    graph
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

fn flood_fill(grid: &mut Vec<Vec<char>>, row: usize, col: usize, loop_indices: &HashSet<usize>, replacement: char, width: usize) {
    if row >= grid.len() || col >= grid[0].len() || loop_indices.contains(&(row * width + col)) || grid[row][col] == replacement {
        return;
    }

    grid[row][col] = replacement;

    flood_fill(grid, row.wrapping_sub(1), col, loop_indices, replacement, width);
    flood_fill(grid, row + 1, col, loop_indices, replacement, width);
    flood_fill(grid, row, col.wrapping_sub(1), loop_indices, replacement, width);
    flood_fill(grid, row, col + 1, loop_indices, replacement, width);
}

fn count_enclosed_tiles(grid: &Vec<Vec<char>>, graph: &UnGraph<usize, ()>) -> usize {
    let mut grid_copy = grid.clone();
    let height = grid.len();
    let width = grid[0].len();

    // Collect all indices from the graph nodes
    let loop_indices: HashSet<_> = graph.node_indices().map(|n| graph[n]).collect();

    for &idx in &loop_indices {
        let row = idx / width;
        let col = idx % width;
        grid_copy[row][col] = '*';
    }

    // Flood fill from all edges of the grid to mark outside area
    for col in 0..width {
        flood_fill(&mut grid_copy, 0, col, &loop_indices, 'o', width); // Top edge
        flood_fill(&mut grid_copy, height - 1, col, &loop_indices, 'o', width); // Bottom edge
    }
    for row in 0..height {
        flood_fill(&mut grid_copy, row, 0, &loop_indices, 'o', width); // Left edge
        flood_fill(&mut grid_copy, row, width - 1, &loop_indices, 'o', width); // Right edge
    }

    // Print the filled grid for visualization
    print_grid(&grid_copy);

    // Count the unmarked tiles
    grid_copy.iter()
        .flatten()
        .filter(|&&c| c != '*' && c != 'o')
        .count()


}

fn _visualize_grid_with_path(grid: &Vec<Vec<char>>, graph: &UnGraph<usize, ()>) {
    let grid_width = grid[0].len();
    let grid_height = grid.len();

    // Create a set of loop nodes for quick lookup
    let loop_nodes: HashSet<usize> = graph.node_indices()
        .map(|node_index| graph[node_index])
        .collect();
    println!();
    for i in 0..grid_height {
        for j in 0..grid_width {
            let linear_index = i * grid_width + j;
            let tile = grid[i][j];

            // Check if this position is part of the loop and not the start position
            if loop_nodes.contains(&linear_index)  {
                print!("*");
            } else {
                print!("{}", tile);
            }
        }
        println!(); // New line at the end of each row
    }
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
    let enclosed_tiles = count_enclosed_tiles(&grid, &path);

        assert_eq!(enclosed_tiles, 4);
    }



    #[test]
    fn test_count_enclosed_tiles_example2() {
        let grid = load_character_grid("bigger_grid_input.txt");
        let path = build_path(&grid);
        let enclosed_tiles = count_enclosed_tiles(&grid, &path);
        assert_eq!(enclosed_tiles, 10);
    }

    #[test]
    fn test_count_enclosed_tiles_example() {
        let grid = load_character_grid("second_test_input.txt");
        let path = build_path(&grid);
        let enclosed_tiles = count_enclosed_tiles(&grid, &path);
        assert_eq!(enclosed_tiles, 8);
    }

    fn visualize_grid_with_path(grid: &Vec<Vec<char>>, graph: &UnGraph<usize, ()>) {
        let grid_width = grid[0].len();
        let grid_height = grid.len();

        // Create a set of loop nodes for quick lookup
        let loop_nodes: HashSet<usize> = graph.node_indices()
            .map(|node_index| graph[node_index])
            .collect();

        for i in 0..grid_height {
            for j in 0..grid_width {
                let linear_index = i * grid_width + j;
                let tile = grid[i][j];

                // Check if this position is part of the loop and not the start position
                if loop_nodes.contains(&linear_index)  {
                    print!("*");
                } else {
                    print!("{}", tile);
                }
            }
            println!(); // New line at the end of each row
        }
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
        let graph = build_path(&grid);

        // Nodes expected to be in the graph
        let expected_nodes: HashSet<usize> = vec![
            6,  // 'S'
            7,  // '-'
            8,  // '7'
            11, // '|'
            13, // '|'
            16, // 'L'
            17, // '-'
            18, // 'J'
        ].into_iter().collect();

        visualize_grid_with_path(&grid, &graph);

        // Check if all expected nodes are present
        for node_index in graph.node_indices() {
            let node = graph[node_index];
            assert!(expected_nodes.contains(&node), "Unexpected node in graph: {}", node);
        }

        // Check if the number of nodes matches the expected
        assert_eq!(graph.node_count(), expected_nodes.len(), "Incorrect number of nodes in graph");

        // You can add additional checks here to validate the edges and their connections
    }


}