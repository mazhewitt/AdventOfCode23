use std::collections::{ HashSet};
use std::fs;
use geo::algorithm::contains::Contains;
use geo::{Polygon, Point};




fn main() {
    let grid = load_character_grid("input.txt");
    let path = build_path(&grid);
    let furthest_distance = (path.len() + (path.len() % 2)) / 2;
    println!("Furthest distance: {}", furthest_distance);
    let enclosed_tiles = count_enclosed_tiles(&path);
    println!("Enclosed tiles: {}", enclosed_tiles);



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



fn count_enclosed_tiles(path: &Vec<(usize, usize)>) -> isize {
    let i_path: Vec<(isize, isize)> = path.iter()
        .map(|&(x, y)| (x as isize, y as isize))
        .collect();
    let area = apply_shoelace_formula(&i_path);
    let boundary_points = count_boundary_points(&i_path);
    (area - boundary_points / 2)
}
fn apply_shoelace_formula(path: &[(isize, isize)]) -> isize {
    let n = path.len();
    if n < 3 {
        // A polygon must have at least 3 vertices
        return 0;
    }

    let mut area = 0;
    for i in 0..n {
        let (_x, y) = path[i];
        let x_prev = path[(i + n - 1) % n].0; // x-coordinate of the previous vertex
        let x_next = path[(i + 1) % n].0;     // x-coordinate of the next vertex

        area += y * (x_prev - x_next);
    }

    (area / 2).abs()
}

// The `apply_picks_theorem`


// Count unique boundary points, considering the path is closed.
fn count_boundary_points(path: &[(isize, isize)]) -> isize {
    let mut unique_points = path.to_vec();
    unique_points.pop(); // Remove the duplicate point at the end
    unique_points.sort();
    unique_points.dedup();
    unique_points.len() as isize
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

    let enclosed_tiles = count_enclosed_tiles(&path);

        assert_eq!(enclosed_tiles, 4);
    }


}