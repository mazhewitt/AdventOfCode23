use std::collections::{HashMap, HashSet};
use std::fs;
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::Dfs;


fn main() {
    let grid = load_character_grid("input.txt");
    let graph = build_graph(&grid);
    let start_pos = find_index_of_s(&grid).unwrap(); // Replace with your start position
    let furthest_distance = find_furthest_point(&graph, start_pos);
    println!("Furthest distance: {}", furthest_distance);

    let enclosed_tiles = count_enclosed_tiles(&grid);
    println!("Enclosed tiles: {}", enclosed_tiles);
}

fn count_enclosed_tiles(p0: &Vec<Vec<char>>) -> usize {
    todo!()
}


fn load_character_grid(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_connected(node: char, node_pos: usize, width: usize, height: usize) -> Vec<usize> {
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
            // For 'S', we do not know the exact connections, so you might need to handle this case differently based on your problem statement
            vec![]
        },
        _ => vec![], // Handle unexpected characters
    }
}


fn build_graph(grid: &Vec<Vec<char>>) -> UnGraph<usize, ()> {
    let width = grid[0].len();
    let height = grid.len();
    let mut graph = UnGraph::new_undirected();

    let s_pos = find_start_position(&grid);
    graph.add_node(s_pos);
    if let Some(connected_node) = find_connected_to_s(&grid, s_pos, width, height) {
        traverse(&mut graph, &grid, connected_node, width, height);
    }

    graph
}

fn traverse(
    graph: &mut UnGraph<usize, ()>,
    grid: &[Vec<char>],
    node_pos: usize,
    width: usize,
    height: usize,
) {
    // Check if the node already exists in the graph
    let current_index = match graph.node_indices().find(|&i| graph[i] == node_pos) {
        Some(index) => index,
        None => graph.add_node(node_pos),
    };

    let row = node_pos / width;
    let col = node_pos % width;
    let node = grid[row][col];
    let connected_nodes = find_connected(node, node_pos, width, height);

    for neighbor_pos in connected_nodes {
        let neighbor_index = match graph.node_indices().find(|&i| graph[i] == neighbor_pos) {
            Some(index) => index,
            None => graph.add_node(neighbor_pos),
        };
        if !graph.contains_edge(current_index, neighbor_index) {
            graph.add_edge(current_index, neighbor_index, ());
        }
        traverse(graph, grid, neighbor_pos, width, height);
    }
}

fn find_start_position(grid: &[Vec<char>]) -> usize {
    grid.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(move |(j, &cell)| {
            if cell == 'S' { Some(i * grid[0].len() + j) } else { None }
        })
    }).next().unwrap()
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



fn find_furthest_point(graph: &UnGraph<usize, ()>, start_index: usize) -> usize {
    let start_node_index = NodeIndex::new(start_index);
    let mut current_node_index = start_node_index;
    let mut previous_node_index: Option<NodeIndex> = None;
    let mut total_distance = 0;
    println!("Start index: {:?}", start_index);

    loop {
        let mut neighbors = graph.neighbors(current_node_index).detach();

        while let Some((_, neighbor_index)) = neighbors.next(graph) {
            if Some(neighbor_index) != previous_node_index {
                previous_node_index = Some(current_node_index);
                current_node_index = neighbor_index;
                total_distance += 1;
                break;
            }
        }

        if current_node_index == start_node_index {
            // Subtract the last increment as we've returned to the start
            total_distance -= 1;
            break;
        }
    }

    // Round up if the total distance is odd
    (total_distance + (total_distance % 2)) / 2
}

fn find_connected_to_s(grid: &[Vec<char>], s_pos: usize, width: usize, height: usize) -> Option<usize> {
    let row = s_pos / width;
    let col = s_pos % width;

    // Check each neighbor of 'S' to find a connected node
    let neighbors = [
        (row.wrapping_sub(1), col), // North
        (row + 1, col),             // South
        (row, col.wrapping_sub(1)), // West
        (row, col + 1),             // East
    ];

    for &(r, c) in &neighbors {
        if r < height && c < width {
            let pos = r * width + c;
            let node = grid[r][c];
            // Check if this node is a valid connection
            if find_connected(node, pos, width, height).contains(&s_pos) {
                return Some(pos);
            }
        }
    }

    None
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
            vec!['.', '|', 'F', '-', '-', '-', '-', '7', '|', '.', '.'],
            vec!['.', '|', '|', '.', '.', '.', '.', '.', '|', '|', '.'],
            vec!['.', '|', '|', '.', '.', '.', '.', '.', '|', '|', '.'],
            vec!['.', '|', 'L', '-', '7', '.', 'F', '-', 'J', '|', '.'],
            vec!['.', '|', '.', '.', '|', '.', '|', '.', '.', '|', '.'],
            vec!['.', 'L', '-', '-', 'J', '.', 'L', '-', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        let enclosed_tiles = count_enclosed_tiles(&grid);

        assert_eq!(enclosed_tiles, 4);
    }



    #[test]
    fn test_count_enclosed_tiles_example() {
        let grid = vec![
            vec!['F', 'F', '7', 'F', 'S', 'F', '7', 'F', '7', 'F', '7', 'F', '7', 'F', '-', '-', '-', '7'],
            vec!['L', '|', 'L', 'J', '|', '|', '|', '|', '|', '|', '|', '|', '|', '|', '|', 'F', '-', '-', 'J'],
            vec!['F', 'L', '-', '7', 'L', 'J', 'L', 'J', '|', '|', '|', '|', '|', 'L', 'J', 'L', '-', '7', '7'],
            vec!['F', '-', '-', 'J', 'F', '-', '-', '7', '|', '|', 'L', 'J', 'L', 'J', '7', 'F', '7', 'F', 'J', '-'],
            vec!['L', '-', '-', '-', 'J', 'F', '-', 'J', 'L', 'J', '.', '|', '|', '-', 'F', 'J', 'L', 'J', 'J', '7'],
            vec!['|', 'F', '|', 'F', '-', 'J', 'F', '-', '-', '-', '7', 'F', '7', '-', 'L', '7', 'L', '|', '7', '|'],
            vec!['|', 'F', 'F', 'J', 'F', '7', 'L', '7', 'F', '-', 'J', 'F', '7', '|', 'J', 'L', '-', '-', '-', '7'],
            vec!['7', '-', 'L', '-', 'J', 'L', '7', '|', '|', 'F', '7', '|', 'L', '7', 'F', '-', '7', 'F', '7', '|'],
            vec!['L', '.', 'L', '7', 'L', 'F', 'J', '|', '|', '|', '|', '|', 'F', 'J', 'L', '7', '|', '|', 'L', 'J'],
            vec!['L', '7', 'J', 'L', 'J', 'L', '-', 'J', 'L', 'J', 'L', 'J', 'L', '-', '-', 'J', 'L', 'J', '.', 'L'],
        ];

        let enclosed_tiles = count_enclosed_tiles(&grid);
        assert_eq!(enclosed_tiles, 10);
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
        let graph = build_graph(&grid);

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