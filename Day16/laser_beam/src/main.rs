use std::collections::{HashSet, VecDeque};
use std::fs;
use std::io::{BufRead};
fn main() {
    let filename = "input.txt";

}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Beam{
    row: isize,
    col: isize,
    d_row : isize,
    d_col : isize
}
impl Beam{
    fn new (r:isize,c:isize,dr:isize,dc:isize) -> Beam{
        let beam = Beam{
            row: r,
            col: c,
            d_row: dr,
            d_col: dc
        };
        beam
    }
}

fn load_character_grid(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_visited_cells(grid: Vec<Vec<char>>) -> usize {
    let mut beam_path: VecDeque<Beam> = VecDeque::new();
    let mut energised_cells: HashSet<Beam> = HashSet::new();
    let start = Beam::new(0, -1, 0, 1);
    beam_path.push_back(start);
    while (!beam_path.is_empty()) {
        let beam = beam_path.pop_front().unwrap();
        let mut next_beam = Beam::new(beam.row + beam.d_row, beam.col + beam.d_col, beam.d_row, beam.d_col);

        if ((next_beam.row) < 0 || (next_beam.row) as usize >= grid.len() || (next_beam.col) < 0 || (next_beam.col) as usize >= grid[0].len()) {
            continue;
        }
        let next_cell = grid[next_beam.row as usize][next_beam.col as usize];
        if (next_cell == '.') || (next_cell == '-' && next_beam.d_col == 0) || (next_cell == '|' && next_beam.d_row != 0) {
            if (!energised_cells.contains(&next_beam)) {
                energised_cells.insert(next_beam);
                beam_path.push_back(next_beam);
            }
        } else if next_cell == '/' {
            let old_dr = next_beam.d_row;
            next_beam.d_row = -next_beam.d_col;
            next_beam.d_col = -old_dr;
            if (!energised_cells.contains(&next_beam)) {
                energised_cells.insert(next_beam);
                beam_path.push_back(next_beam);
            }
        } else if next_cell == '\\' {
            let old_dr = next_beam.d_row;
            next_beam.d_row = next_beam.d_col;
            next_beam.d_col = old_dr;
            if (!energised_cells.contains(&next_beam)) {
                energised_cells.insert(next_beam);
                beam_path.push_back(next_beam);
            }
        } else {
            // we have hit a splitter
            if (next_cell == '|') {
                let mut beam1 = Beam::new(next_beam.row + 1, next_beam.col, 1, 0);
                let mut beam2 = Beam::new(next_beam.row - 1, next_beam.col, -1, -0);
                if (!energised_cells.contains(&beam1)) {
                    energised_cells.insert(beam1);
                    beam_path.push_back(beam1);
                }
                if (!energised_cells.contains(&beam2)) {
                    energised_cells.insert(beam2);
                    beam_path.push_back(beam2);
                }
            } else {
                let mut beam1 = Beam::new(next_beam.row, next_beam.col, 0, -1);
                let mut beam2 = Beam::new(next_beam.row, next_beam.col, 0, 1);
                if (!energised_cells.contains(&beam1)) {
                    energised_cells.insert(beam1);
                    beam_path.push_back(beam1);
                }
                if (!energised_cells.contains(&beam2)) {
                    energised_cells.insert(beam2);
                    beam_path.push_back(beam2);
                }
            }
        }
    }
    let visited: HashSet<(isize, isize)> = energised_cells
        .iter()
        .map(|beam| (beam.row, beam.col))
        .collect();
    visited.len()

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn energise_cells(){
        let filename = "test.txt";
        let grid=  load_character_grid(filename);
        let v = calculate_visited_cells(grid);
        assert_eq!(v, 46);

    }


}