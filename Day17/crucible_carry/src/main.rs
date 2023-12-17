use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

fn main() {

    let grid = load_character_grid("test.txt");
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
    println!("starting");
    let mut heap:BinaryHeap<(isize,Node)> = BinaryHeap::new();
    let mut visited: HashSet<Node> = HashSet::new();
    heap.push((0,Node { row: 0, col: 0, dir: Direction::None, indir: 0}));

    loop {
        let (heat_loss, head) = heap.pop().unwrap();
        println ! ("arrived at {} {} with hl {} heading: {:?}",  head.row, head.col,heat_loss, head.dir);
        if  head.row == grid.len() as isize - 1 && head.col == grid[0].len() as isize - 1 {
            return heat_loss*-1;
        }
        visited.insert(head);
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
            .iter()
            .for_each(|dir| {
                let (row, col) = match dir {
                    Direction::Up => (head.row - 1, head.col),
                    Direction::Down => (head.row + 1, head.col),
                    Direction::Left => (head.row, head.col - 1),
                    Direction::Right => (head.row, head.col + 1),
                    _ => panic!("Invalid direction")
                };
                if !(*dir == Direction::Up && head.dir == Direction::Down) ||
                    (*dir == Direction::Down && head.dir == Direction::Up) ||
                    (*dir == Direction::Left && head.dir == Direction::Right) ||
                    (*dir == Direction::Right && head.dir == Direction::Left) {

                    if row < grid.len() as isize && col < grid[0].len() as isize && row >= 0 && col >= 0 {
                        let new_heat_loss = heat_loss - (grid[row as usize][col as usize] as isize - '0' as isize) ;

                        let new_indir = if head.dir == *dir { head.indir + 1 } else { 1 };
                        if new_indir <= 3 {
                            let new_node = Node { row, col, dir: *dir, indir: new_indir };
                            if !visited.contains(&new_node) {
                                heap.push((new_heat_loss, new_node));
                            }
                        }
                    }
                }
            });
    }
}
