use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

fn main() {

    let grid = load_character_grid("input.txt");
    let least_enery_loss = calculate_least_energy_loss(&grid);
    println!("least energy loss: {}", least_enery_loss);
}
#[derive(Eq, Debug, Copy, Clone, PartialEq, Hash, PartialOrd, Ord)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
    None = 0,
}


fn load_character_grid(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    row: isize,
    col: isize,
    dir: Direction,
    indir: usize,
}



fn calculate_least_energy_loss(grid: &Vec<Vec<char>>) -> isize {
    let mut heap:BinaryHeap<(isize,Node)> = BinaryHeap::new();
    let mut visited: HashSet<Node> = HashSet::new();
    heap.push((0,Node { row: 0, col: 0, dir: Direction::Right, indir: 0}));
    heap.push((0,Node { row: 0, col: 0, dir: Direction::Down, indir: 0}));
    let width = grid[0].len() as isize;
    let height = grid.len() as isize;
    loop {
        let (heat_loss, head) = heap.pop().unwrap();
        //println!("arrived at {} {} with hl {} heading: {:?}, with indir of {}", head.row, head.col, heat_loss, head.dir, head.indir);

        // Check for goal condition
        if head.row == (height - 1)  && head.col == (width - 1) && head.indir >=4 {
            return -heat_loss;
        }

        if visited.contains(&head) {
            continue;
        }

        visited.insert(head);
        if head.indir < 4{ // must head in the same direction

            let (row, col) = match next_cell_given_direction(width, height, head, head.dir) {
                Some(value) => value,
                None => continue,
            };
            let new_heat_loss = heat_loss - (grid[row as usize][col as usize] as isize - '0' as isize);
            let new_indir= head.indir + 1;
            heap.push((new_heat_loss, Node { row, col, dir:head.dir, indir: new_indir }));
        }
        else {
            for &dir in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                let (row, col) = match next_cell_given_direction(width, height, head, dir) {
                    Some(value) => value,
                    None => continue,
                };

                // Check for reverse direction
                if !((dir == Direction::Up && head.dir == Direction::Down) ||
                    (dir == Direction::Down && head.dir == Direction::Up) ||
                    (dir == Direction::Left && head.dir == Direction::Right) ||
                    (dir == Direction::Right && head.dir == Direction::Left)) {
                    let new_heat_loss = heat_loss - (grid[row as usize][col as usize] as isize - '0' as isize);
                    let new_indir = if head.dir == dir { head.indir + 1 } else { 1 };
                    if new_indir <= 10 {
                        heap.push((new_heat_loss, Node { row, col, dir, indir: new_indir }));
                    }
                }
            }
        }
    }
}

fn next_cell_given_direction(width: isize, height: isize, head: Node, dir: Direction) -> Option<(isize, isize)> {
    Some(match dir {
        Direction::Up if head.row > 0 => (head.row - 1, head.col),
        Direction::Down if head.row < (height - 1) => (head.row + 1, head.col),
        Direction::Left if head.col > 0 => (head.row, head.col - 1),
        Direction::Right if head.col < (width - 1) => (head.row, head.col + 1),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_least_energy_loss() {
        let grid = load_character_grid("test.txt");
        let least_enery_loss = calculate_least_energy_loss(&grid);
        assert_eq!(least_enery_loss,94);
    }
}
