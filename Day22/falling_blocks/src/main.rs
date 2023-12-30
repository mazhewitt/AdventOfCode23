use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Brick {
    reference: String,
    left_x: usize,
    front_y: usize,
    bottom_z: usize,
    x_width: usize,
    z_height: usize,
    y_depth: usize,
}


impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Brick) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bottom_z.cmp(&other.bottom_z)
            .then_with(|| self.left_x.cmp(&other.left_x))  // Tie-breaker with left_x
            .then_with(|| self.front_y.cmp(&other.front_y))  // Next tie-breaker with front_y
            .then_with(|| self.z_height.cmp(&other.z_height))  // Next tie-breaker with z_height
            .then_with(|| self.x_width.cmp(&other.x_width))  // Next tie-breaker with x_width
            .then_with(|| self.y_depth.cmp(&other.y_depth))  // Next tie-breaker with y_depth
    }
}

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        let z_overlap = (self.bottom_z >= other.bottom_z && self.bottom_z < (other.bottom_z + other.z_height))
            || (other.bottom_z >= self.bottom_z && other.bottom_z < (self.bottom_z + self.z_height));
                println!("z_overlap: {}", z_overlap);
        self.overlaps_x_y(&other) && z_overlap
    }
    fn overlaps_x_y(&self, other: &Brick) -> bool {
        let x_overlap = (self.left_x >= other.left_x && self.left_x < (other.left_x + other.x_width))
            || (other.left_x >= self.left_x && other.left_x < (self.left_x + self.x_width));
        let y_overlap = (self.front_y >= other.front_y && self.front_y < (other.front_y + other.y_depth))
            || (other.front_y >= self.front_y && other.front_y < (self.front_y + self.y_depth));
        x_overlap && y_overlap
    }

    fn supports(&self, other: &Brick) -> bool {
        self.overlaps_x_y(other) && self.bottom_z + self.z_height == other.bottom_z
    }
}

fn parse_brick(brick_str: &str) -> Brick {
    let coords: Vec<usize> = brick_str.split(['~', ','].as_ref())
        .map(|s| s.parse().unwrap())
        .collect();

    Brick {
        reference: brick_str.to_string(),
        left_x: coords[0].min(coords[3]),
        front_y: coords[1].min(coords[4]),
        bottom_z: coords[2].min(coords[5]),
        x_width: (coords[0] as isize - coords[3] as isize).abs() as usize + 1,
        y_depth: (coords[1] as isize - coords[4] as isize).abs() as usize + 1,
        z_height: (coords[2] as isize - coords[5] as isize).abs() as usize + 1,
    }
}

fn add_brick(set: &mut HashSet<Brick>, new_brick: Brick) -> bool {
    // we have to check the assumption that the brick don't overlap at the start

    for brick in set.iter() {
        let overlap = brick.overlaps(&new_brick);
        assert!(!overlap, "Brick {:?} overlaps with {:?}", new_brick, brick)
    }
    // If it doesn't overlap, add to set and return true
    set.insert(new_brick);
    true
}

fn simulate_fall(mut bricks: Vec<Brick>) -> BTreeSet<Brick>{
    let mut yet_to_fall: BTreeSet<Brick> = bricks.into_iter().collect(); // all bricks start here
    let mut fallen: BTreeSet<Brick> = BTreeSet::new(); // no bricks have fallen initially

    while let Some(brick) = yet_to_fall.pop_first() { // assuming pop_first() retrieves and removes the lowest item
        let new_position = find_fall_position(&brick, &fallen);
        fallen.insert(new_position);
    }
    fallen
}

fn find_fall_position(brick: &Brick, fallen: &BTreeSet<Brick>) -> Brick {
    // Assuming bricks can't go below ground level
    if brick.bottom_z == 1 {
        return brick.clone(); // The brick is already on the ground
    }

    // Start from just above the ground or where it currently is
    let mut highest_supported_z = 1;

    // Check against each fallen brick to find the highest position it can fall to
    for fallen_brick in fallen.iter() {
        if brick.overlaps_x_y(fallen_brick) {
            // Calculate the potential new z position (just above the fallen brick)
            let potential_new_z = fallen_brick.bottom_z + fallen_brick.z_height;
            if potential_new_z < brick.bottom_z && potential_new_z > highest_supported_z {
                highest_supported_z = potential_new_z;
            }
        }
    }
    // Create a new brick with the new bottom_z position
    let mut new_position = brick.clone();
    new_position.bottom_z = highest_supported_z;
    new_position
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_brick(){
        let test_brick_str = "2,2,2~2,2,2";
        let test_brick = Brick{reference:"2,2,2~2,2,2".to_string(), left_x: 2, front_y: 2, bottom_z: 2, x_width: 1, z_height: 1, y_depth: 1};
        let brick = parse_brick(test_brick_str);
        assert_eq!(brick, test_brick);
    }

    #[test]
    fn test_parse_brick2(){
        let test_brick_str = "0,0,1~0,0,10";
        let test_brick = Brick{reference:"0,0,1~0,0,10".to_string(),left_x: 0, front_y: 0, bottom_z: 1, x_width: 1, z_height: 10, y_depth: 1};
        let brick = parse_brick(test_brick_str);
        assert_eq!(brick, test_brick);
    }

    #[test]
    fn test_parse_test_file(){
        let filename = "test.txt";
        let file = File::open(filename).expect("file not found");
        let reader = BufReader::new(file);
        let mut brick_set: HashSet<Brick> = HashSet::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let brick = parse_brick(&line);
            add_brick(&mut brick_set, brick);
        }
        assert_eq!(brick_set.len(), 7);
    }



    #[test]
    fn no_overlap() {
        let brick1 = Brick {
            reference:"brick1".to_string(),
            left_x: 0,
            front_y: 0,
            bottom_z: 0,
            x_width: 2,
            z_height: 2,
            y_depth: 2,
        };
        let brick2 = Brick {
            reference:"brick2".to_string(),
            left_x: 3,
            front_y: 3,
            bottom_z: 3,
            x_width: 2,
            z_height: 2,
            y_depth: 2,
        };
        assert!(!brick1.overlaps(&brick2));
    }

    #[test]
    fn overlap_one_dimension() {
        let brick1 = Brick {
            reference:"brick1".to_string(),
            left_x: 0,
            front_y: 0,
            bottom_z: 0,
            x_width: 3,
            z_height: 2,
            y_depth: 2,
        };
        let brick2 = Brick {
            reference:"brick2".to_string(),
            left_x: 2,
            front_y: 3,
            bottom_z: 3,
            x_width: 2,
            z_height: 2,
            y_depth: 2,
        };
        assert!(!brick1.overlaps(&brick2));
    }

    #[test]
    fn overlap_two_dimensions() {
        let brick1 = Brick {
            reference:"brick1".to_string(),
            left_x: 0,
            front_y: 0,
            bottom_z: 0,
            x_width: 3,
            z_height: 3,
            y_depth: 3,
        };
        let brick2 = Brick {
            reference:"brick2".to_string(),
            left_x: 2,
            front_y: 2,
            bottom_z: 3,
            x_width: 2,
            z_height: 2,
            y_depth: 2,
        };
        assert!(!brick1.overlaps(&brick2));
    }

    #[test]
    fn complete_overlap() {
        let brick1 = Brick {
            reference:"brick1".to_string(),
            left_x: 1,
            front_y: 1,
            bottom_z: 1,
            x_width: 3,
            z_height: 3,
            y_depth: 3,
        };
        let brick2 = Brick {
            reference:"brick2".to_string(),
            left_x: 2,
            front_y: 2,
            bottom_z: 2,
            x_width: 1,
            z_height: 1,
            y_depth: 1,
        };
        println!("brick1: {:?}", brick1);
        println!("brick2: {:?}", brick2);
        assert!(brick1.overlaps(&brick2));
    }
    #[test]
    fn brick_2_lands_on_brick_1() {
        let brick1 = Brick {
            reference:"brick1".to_string(),
            left_x: 1,
            front_y: 1,
            bottom_z: 1,
            x_width: 3,
            z_height: 3,
            y_depth: 3,
        };
        let brick2 = Brick {
            reference:"brick2".to_string(),
            left_x: 2,
            front_y: 2,
            bottom_z: 9,
            x_width: 1,
            z_height: 1,
            y_depth: 1,
        };
        let bricks = vec![brick2, brick1];
        let fallen = simulate_fall(bricks);
        // find "brick1" in fallen
        let f_brick1 = fallen.iter().find(|b| b.reference == "brick1").unwrap();
        let f_brick2 = fallen.iter().find(|b| b.reference == "brick2").unwrap();
        assert_eq!(f_brick1.bottom_z, 1);
        assert_eq!(f_brick2.bottom_z, 4);


    }
    #[test]
    fn brick_2_lands_next_to_brick_1() {
        let brick1 = Brick {
            reference:"brick1".to_string(),
            left_x: 1,
            front_y: 1,
            bottom_z: 1,
            x_width: 3,
            z_height: 3,
            y_depth: 3,
        };
        let brick2 = Brick {
            reference:"brick2".to_string(),
            left_x: 4,
            front_y: 2,
            bottom_z: 9,
            x_width: 1,
            z_height: 1,
            y_depth: 1,
        };
        let bricks = vec![brick1, brick2];
        let mut fallen = simulate_fall(bricks);
        let f_brick1 = fallen.pop_first().unwrap();
        let f_brick2 = fallen.pop_first().unwrap();
        assert_eq!(f_brick1.bottom_z, 1);
        assert_eq!(f_brick2.bottom_z, 1);


    }
    #[test]
    fn test_all_bricks(){
        let filename = "test.txt";
        let file = File::open(filename).expect("file not found");
        let reader = BufReader::new(file);
        let mut bricks: Vec<Brick> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let brick = parse_brick(&line);
            bricks.push(brick);
        }
        println!("A: {:?}", bricks[0]);
        println!("B: {:?}", bricks[1]);
        println!("C: {:?}", bricks[2]);
        assert!(bricks[0].overlaps_x_y(&bricks[1]));
        assert!(!bricks[1].overlaps_x_y(&bricks[2]));


        let fallen = simulate_fall(bricks);
        assert_eq!(fallen.len(), 7);
        let brick_a = fallen.iter().find(|b| b.reference == "1,0,1~1,2,1").unwrap();
        let brick_b = fallen.iter().find(|b| b.reference == "0,0,2~2,0,2").unwrap();
        let brick_c = fallen.iter().find(|b| b.reference == "0,2,3~2,2,3").unwrap();
        println!("a: {:?}", brick_a);
        println!("b: {:?}", brick_b);
        println!("c: {:?}", brick_c);
        assert!(brick_a.overlaps_x_y(&brick_b));
        assert!(brick_a.supports(&brick_b));
    }

    #[test]
    fn test_overlaps_x_y(){
        let brick1 = Brick {
            reference:"brick1".to_string(),
            left_x: 1,
            front_y: 0,
            bottom_z: 1,
            x_width: 1,
            z_height: 1,
            y_depth: 3,
        };
        let brick2 = Brick {
            reference:"brick2".to_string(),
            left_x: 0,
            front_y: 0,
            bottom_z: 2,
            x_width: 3,
            z_height: 1,
            y_depth: 1,
        };
        println!("brick1: {:?}", brick1);
        println!("brick2: {:?}", brick2);
        assert!(brick1.overlaps_x_y(&brick2));
    }
    #[test]
    fn test_a_supports_b(){
        let brickA = Brick {
            reference:"brickA".to_string(),
            left_x: 1,
            front_y: 0,
            bottom_z: 1,
            x_width: 1,
            z_height: 1,
            y_depth: 3,
        };
        let brickB = Brick {
            reference:"brickB".to_string(),
            left_x: 0,
            front_y: 0,
            bottom_z: 2,
            x_width: 3,
            z_height: 1,
            y_depth: 1,
        };
        println!("brick1: {:?}", brickA);
        println!("brick2: {:?}", brickB);
        assert!(brickA.supports(&brickB));
    }

    #[test]
    fn test_b_falls_on_a(){
        let brickA = Brick {
            reference:"brickA".to_string(),
            left_x: 1,
            front_y: 0,
            bottom_z: 1,
            x_width: 1,
            z_height: 1,
            y_depth: 3,
        };
        let brickB = Brick {
            reference:"brickB".to_string(),
            left_x: 0,
            front_y: 0,
            bottom_z: 2,
            x_width: 3,
            z_height: 1,
            y_depth: 1,
        };
        println!("brick1: {:?}", brickA);
        println!("brick2: {:?}", brickB);
        let mut fallen = BTreeSet::new();
        fallen.insert(brickA.clone());
        let fallen_b = find_fall_position(&brickB, &fallen);

        assert!(brickA.supports(&fallen_b));
    }

}