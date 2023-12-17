use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

fn main() {
    println!("Hello, world!");
}

fn load_character_grid(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Eq, PartialEq, Ord, Debug)]
struct Node {
    row: usize,
    col: usize,
    dir: i32,
    indir: i32,
    heat_loss: i32,  // Added distance field
}


impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss == other.heat_loss && self.row == other.row && self.col == other.col && self.dir == other.dir && self.indir == other.indir
    }
}
impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(other.heat_loss.cmp(&self.heat_loss))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

fn calculate_least_energy_loss(grid: &Vec<Vec<char>>) -> usize {
    let node_heap:BinaryHeap<Node> = BinaryHeap::new();
    let mut visited : HashSet<(usize,usize)>= HashSet::new();
    let mut position: (usize, usize) = (0,0);
    while (position.0 < grid.len()-1) && (position.1 < grid[0].len()-1) {
        let mut node = Node{
            row: position.0,
            col: position.1,
            dir: 0,
            indir: 0,
            heat_loss: 0
        };
        node_heap.push(node);
        position.0 += 1;
        position.1 += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let grid = load_character_grid("input.txt");
        let least_enery_loss = calculate_least_energy_loss(&grid);
        assert_eq!(least_enery_loss, "102");
    }


}