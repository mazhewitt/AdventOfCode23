 use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let _filename = "input.txt";
    let grid = load_character_grid(_filename);
    let v = calculate_visited_cells(&grid, 0, 0, 0, 1);
    println!("Visited cells: {}", v);
    let max_energized = find_best_configuration(&grid);
    println!("Max energized: {}", max_energized);
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

fn calculate_visited_cells(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize, d_row: isize, d_col: isize) -> usize {
    let mut beam_path: VecDeque<Beam> = VecDeque::new();
    let mut energised_cells: HashSet<Beam> = HashSet::new();
    let start = Beam::new(start_row as isize, start_col as isize, d_row, d_col);
    beam_path.push_back(start);
    energised_cells.insert(start);
    while !beam_path.is_empty() {
        let beam = beam_path.pop_front().unwrap();
        process_beam(&grid, &mut beam_path, &mut energised_cells, beam);
    }

    let mut set: HashSet<(isize, isize)> = HashSet::new();
    for s_beam in energised_cells {
        set.insert((s_beam.row, s_beam.col));
    }
    set.len()
}

fn process_beam(grid: &Vec<Vec<char>>, beam_path: &mut VecDeque<Beam>, energised_cells: &mut HashSet<Beam>, beam: Beam) {

    let next_cell = grid[beam.row as usize][beam.col as usize];
    let next_beams = match next_cell {
        '.' => handle_empty_space(beam),
        '/' | '\\' => handle_mirror(beam, next_cell),
        '|' | '-' => handle_splitter(beam, next_cell),
        _ => panic!("Invalid cell: {}", next_cell),
    };
    for beam in next_beams {
        if !energised_cells.contains(&beam) && is_within_bounds(&beam, grid){
            energised_cells.insert(beam);
            beam_path.push_back(beam);
        }
    }
}

fn is_within_bounds(beam: &Beam, grid: &Vec<Vec<char>>) -> bool {
    beam.row >= 0 && (beam.row as usize) < grid.len() && beam.col >= 0 && (beam.col as usize) < grid[0].len()
}

fn handle_empty_space(beam: Beam) -> Vec<Beam> {
    // Move the beam to the next position
    let next_beam = Beam::new(beam.row + beam.d_row, beam.col + beam.d_col, beam.d_row, beam.d_col);
    vec![next_beam]
}

fn handle_mirror( beam: Beam, mirror_type: char) -> Vec<Beam>{
    let mut next_beam = beam.clone();

    match mirror_type {
        '/' => {
            let old_dc = next_beam.d_col;
            next_beam.d_col = -1 * next_beam.d_row;
            next_beam.d_row = -1 * old_dc;

        },
        '\\' => {
            let old_dr = beam.d_row;
            next_beam.d_row =  next_beam.d_col;
            next_beam.d_col =  old_dr;

        },
        _ => panic!("Invalid mirror type: {}", mirror_type),
    }

    // Move the beam to the next position
    next_beam.row += next_beam.d_row;
    next_beam.col += next_beam.d_col;
    vec![next_beam]

}


fn handle_splitter( beam: Beam, cell:char) -> Vec<Beam>{
    match cell {
        '|' => {
            if beam.d_col == 0 { // Beam is moving vertically
                vec![Beam::new(beam.row+beam.d_row, beam.col, beam.d_row, beam.d_col)]
            } else {
                // Horizontal splitter ('|'), split vertically (up and down)
                vec![
                    Beam::new(beam.row - 1, beam.col, -1, 0),
                    Beam::new(beam.row + 1, beam.col, 1, 0),
                ]
            }
        }
        '-' => {
            if beam.d_row == 0 { // Beam is moving horizontally
                vec![Beam::new(beam.row, beam.col+beam.d_col, beam.d_row, beam.d_col)]
            } else {
                // Vertical splitter ('-'), split horizontally (left and right)
                vec![
                    Beam::new(beam.row, beam.col - 1, 0, -1),
                    Beam::new(beam.row, beam.col + 1, 0, 1),
                ]
            }
        }
        _ => panic!("Invalid splitter: {}", cell),
    }
}

fn find_best_configuration(grid: &Vec<Vec<char>>) -> usize {
    let mut max_energized = 0;

    let height = grid.len();
    let width = grid[0].len();

    // Try each position on the top and bottom rows
    for col in 0..width {
        max_energized = max_energized.max(calculate_visited_cells(grid, 0, col, 1, 0)); // Top row, heading down
        max_energized = max_energized.max(calculate_visited_cells(grid, height - 1, col, -1, 0)); // Bottom row, heading up
    }

    // Try each position on the leftmost and rightmost columns
    for row in 0..height {
        max_energized = max_energized.max(calculate_visited_cells(grid, row, 0, 0, 1)); // Leftmost column, heading right
        max_energized = max_energized.max(calculate_visited_cells(grid, row, width - 1, 0, -1)); // Rightmost column, heading left
    }

    max_energized
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn energise_cells(){
        let filename = "test.txt";
        let grid=  load_character_grid(filename);
        let v = calculate_visited_cells(&grid, 0, 0, 0, 1);
        assert_eq!(v, 46);

    }

    #[test]
    fn test_handle_empty_space() {
        let beam = Beam::new(1, 1, 0, 1); // Moving right
        let next_beam = handle_empty_space(beam)[0];

        assert_eq!(next_beam.row, 1);
        assert_eq!(next_beam.col, 2); // Should have moved one step right
    }

    #[test]
    fn test_handle_mirror_forward_slash() {

        // Beam moving right towards the mirror
        let beam = Beam::new(0, 0, 0, 1);

        // Process the beam with a forward slash mirror
        let new_beam = handle_mirror(beam, '/')[0];

        assert_eq!(new_beam.row, -1); // Moved down one row
        assert_eq!(new_beam.col, 0); // Stayed in the same column
        assert_eq!(new_beam.d_row, -1); // Direction up
        assert_eq!(new_beam.d_col, 0); // No horizontal movement
    }

    #[test]
    fn test_handle_mirror_back_slash() {
        // Beam moving right towards the mirror
        let beam = Beam::new(0, 0, 0, 1);

        let new_beam = handle_mirror(beam, '\\')[0];
        assert_eq!(new_beam.row, 1); // Moved up one row
        assert_eq!(new_beam.col, 0); // Stayed in the same column
        assert_eq!(new_beam.d_row, 1); // Direction up
        assert_eq!(new_beam.d_col, 0); // No horizontal movement
    }

    #[test]
    fn test_generate_split_beams_vertical_splitter() {
        // Beam moving horizontally towards a '|' splitter
        let beam = Beam::new(2, 2, 0, 1);

        // Generate split beams
        let split_beams = handle_splitter(beam, '|');

        // Expect two beams: one moving up and one moving down
        assert_eq!(split_beams.len(), 2);
        assert!(split_beams.contains(&Beam::new(1, 2, -1, 0))); // Upward beam
        assert!(split_beams.contains(&Beam::new(3, 2, 1, 0)));  // Downward beam
    }

    #[test]
    fn test_generate_split_beams_horizontal_splitter() {
        // Beam moving vertically towards a '-' splitter
        let beam = Beam::new(2, 2, 1, 0);

        // Generate split beams
        let split_beams = handle_splitter(beam, '-');

        // Expect two beams: one moving left and one moving right
        assert_eq!(split_beams.len(), 2);
        assert!(split_beams.contains(&Beam::new(2, 1, 0, -1))); // Leftward beam
        assert!(split_beams.contains(&Beam::new(2, 3, 0, 1)));  // Rightward beam
    }
}