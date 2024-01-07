use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let filename = "test.txt";
    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);
    let mut bricks: Vec<Brick> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let brick = parse_brick(&line);
        bricks.push(brick);
    }
    println!("Bricks: {}", bricks.len());
    let fallen = simulate_fall(bricks);
    println!("Fallen: {}", fallen.len());
    let safe_to_remove = count_save_to_remove(&fallen);
    let removable = remove_safe_bricks(fallen);
    // print removable bricks
    for brick in removable.iter() {
        println!("Removable: {:?}", brick);
    }
    println!("Safe to remove: {}, removable: {}", safe_to_remove, removable.len());
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

fn simulate_fall(bricks: Vec<Brick>) -> BTreeSet<Brick>{
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


    let mut highest_supported_z = 1;

    // Check against each fallen brick to find the highest position it can fall to
    for fallen_brick in fallen.iter() {
        if brick.overlaps_x_y(fallen_brick) {
            // Calculate the potential new z position (just above the fallen brick)
            let potential_new_z = fallen_brick.bottom_z + fallen_brick.z_height;
            if potential_new_z <= brick.bottom_z && potential_new_z > highest_supported_z {
                highest_supported_z = potential_new_z;
            }
        }
    }
    // Create a new brick with the new bottom_z position
    let mut new_position = brick.clone();
    new_position.bottom_z = highest_supported_z;
    new_position
}

fn remove_safe_bricks(mut fallen: BTreeSet<Brick>) -> Vec<Brick> {
    let mut safe_to_remove: Vec<Brick> = Vec::new();

// build a graph of all bricks that support each brick

    let (mut support_graph, mut supported_by_graph) = build_support_graphs(&fallen);

    // loop until there are no safe to remove bricks
    loop {
        let mut removed = false;
        for brick in fallen.iter() {
            if is_safe_to_remove(brick, &support_graph, &supported_by_graph) {
                safe_to_remove.push(brick.clone());
                // remove brick from support_graph and supported_by_graph
                support_graph.remove(brick);
                if let Some(supported_bricks) = support_graph.get(brick) {
                    for supported_brick in supported_bricks {
                        // Get the supporters of the currently supported brick
                        if let Some(supporters) = supported_by_graph.get_mut(supported_brick) {
                            // Remove the brick from its supporters
                            supporters.retain(|supporter| supporter != brick); // Retain only supporters that are not the brick
                        }
                    }
                }
                removed = true;
            }
        }
        // retain only bricks that are not in safe_to_remove
        fallen.retain(|brick| !safe_to_remove.contains(brick));
        if !removed {
            break;
        }
    }

    safe_to_remove

}

fn count_save_to_remove(fallen: &BTreeSet<Brick>) -> usize {
    let (support_graph, supported_by_graph) = build_support_graphs(fallen);
    let mut safe_to_remove = 0;
    for brick in fallen.iter() {
        if is_safe_to_remove(brick, &support_graph, &supported_by_graph) {
            safe_to_remove += 1;
        }
    }
    safe_to_remove
}

fn build_support_graphs(fallen: &BTreeSet<Brick>) -> (HashMap<Brick, Vec<Brick>>, HashMap<Brick, Vec<Brick>>) {
    let mut support_graph: HashMap<Brick, Vec<Brick>> = HashMap::new();
    for brick in fallen.iter() {
        for supported_brick in fallen.iter() {
            if brick.supports(supported_brick) {
                let supported_list = support_graph.entry(brick.clone()).or_insert(Vec::new());
                supported_list.push(supported_brick.clone());
            }
        }
    }

    let mut supported_by_graph: HashMap<Brick, Vec<Brick>> = HashMap::new();
    // build a graph of all bricks that are supported by each brick
    for (brick, supported_by) in support_graph.iter() {
        for supported_brick in supported_by.iter() {
            let supported_by_list = supported_by_graph.entry(supported_brick.clone()).or_insert(Vec::new());
            supported_by_list.push(brick.clone());
        }
    }
    (support_graph, supported_by_graph)
}

fn is_safe_to_remove(brick: &Brick,
                     support_graph: &HashMap<Brick, Vec<Brick>>,
                     supported_by_graph: &HashMap<Brick, Vec<Brick>>) -> bool {

    // Get the bricks that the current brick supports
    if let Some(supported_bricks) = support_graph.get(brick) {
        for supported_brick in supported_bricks {
            // Check if the supported brick has other supports
            if let Some(supporters) = supported_by_graph.get(supported_brick) {
                // If the only supporter of the supported brick is the current brick, it's not safe to remove
                if supporters.len() == 1 && supporters.contains(brick) {
                    return false;
                }
            } else {
                // If there are no other supporters for a supported brick, it's not safe to remove
                return false;
            }
        }
    }
    // If all supported bricks have other supporters, it's safe to remove
    true
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
        assert!(brick_a.supports(&brick_c));
        let safe_to_remove = count_save_to_remove(&fallen);
        assert_eq!(safe_to_remove, 5);
    }


    #[test]
    fn test_safe_to_remove(){

    // load bricks from file
        let filename = "test.txt";
        let file = File::open(filename).expect("file not found");
        let reader = BufReader::new(file);
        let mut bricks: Vec<Brick> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let brick = parse_brick(&line);
            bricks.push(brick);
        }

        let mut fallen = simulate_fall(bricks);
        let brick_a = fallen.iter().find(|b| b.reference == "1,0,1~1,2,1").unwrap();
        let brick_b = fallen.iter().find(|b| b.reference == "0,0,2~2,0,2").unwrap();
        let brick_c = fallen.iter().find(|b| b.reference == "0,2,3~2,2,3").unwrap();
        let brick_d = fallen.iter().find(|b| b.reference == "0,0,4~0,2,4").unwrap();
        let brick_e = fallen.iter().find(|b| b.reference == "2,0,5~2,2,5").unwrap();
        let brick_f = fallen.iter().find(|b| b.reference == "0,1,6~2,1,6").unwrap();
        let brick_g = fallen.iter().find(|b| b.reference == "1,1,8~1,1,9").unwrap();


        let (support_graph, supported_by_graph) = build_support_graphs(&fallen);
        println!("Brick A: {:?}", brick_a);
        println!("Brick B: {:?}", brick_b);
        println!("Brick C: {:?}", brick_c);
        assert!(brick_a.supports(&brick_b));
        assert!(brick_a.supports(&brick_c));
        let supported_by_brick_a = support_graph.get(brick_a).unwrap();
        assert_eq!(supported_by_brick_a.len(), 2);
        assert!(supported_by_brick_a.contains(&brick_b));
        assert!(supported_by_brick_a.contains(&brick_c));
        let supporters_of_brick_b = supported_by_graph.get(brick_b).unwrap();
        assert_eq!(supporters_of_brick_b.len(), 1);
        assert!(supporters_of_brick_b.contains(&brick_a));
        //assert!(!is_safe_to_remove(&brick_a, &support_graph, &supported_by_graph));

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
        let brick_a = Brick {
            reference:"brick_a".to_string(),
            left_x: 1,
            front_y: 0,
            bottom_z: 1,
            x_width: 1,
            z_height: 1,
            y_depth: 3,
        };
        let brick_b = Brick {
            reference:"brick_b".to_string(),
            left_x: 0,
            front_y: 0,
            bottom_z: 2,
            x_width: 3,
            z_height: 1,
            y_depth: 1,
        };
        println!("brick1: {:?}", brick_a);
        println!("brick2: {:?}", brick_b);
        assert!(brick_a.supports(&brick_b));
    }

    #[test]
    fn test_b_falls_on_a(){
        let brick_a = Brick {
            reference:"brick_a".to_string(),
            left_x: 1,
            front_y: 0,
            bottom_z: 1,
            x_width: 1,
            z_height: 1,
            y_depth: 3,
        };
        let brick_b = Brick {
            reference:"brick_b".to_string(),
            left_x: 0,
            front_y: 0,
            bottom_z: 2,
            x_width: 3,
            z_height: 1,
            y_depth: 1,
        };
        println!("brick1: {:?}", brick_a);
        println!("brick2: {:?}", brick_b);
        let mut fallen = BTreeSet::new();
        fallen.insert(brick_a.clone());
        let fallen_b = find_fall_position(&brick_b, &fallen);

        assert!(brick_a.supports(&fallen_b));
    }

    #[test]
    fn test_is_safe_to_remove() {
        let brick_a = Brick {
            reference: "brick_a".to_string(),
            left_x: 1,
            front_y: 0,
            bottom_z: 1,
            x_width: 1,
            z_height: 1,
            y_depth: 3,
        };
        let brick_b = Brick {
            reference: "brick_b".to_string(),
            left_x: 0,
            front_y: 0,
            bottom_z: 2,
            x_width: 3,
            z_height: 1,
            y_depth: 1,
        };

        let brick_c = Brick {
            reference: "brick_c".to_string(),
            left_x: 3,
            front_y: 0,
            bottom_z: 1,
            x_width: 1,
            z_height: 1,
            y_depth: 3,
        };
        println!("brick_a: {:?}", brick_a);
        println!("brick_b: {:?}", brick_b);
        println!("brick_c: {:?}", brick_c);
        let mut fallen = BTreeSet::new();
        fallen.insert(brick_a.clone());
        fallen.insert(brick_b.clone());
        fallen.insert(brick_c.clone());
        let (support_graph, supported_by_graph) = build_support_graphs(&fallen);

        assert!(is_safe_to_remove(&brick_a, &support_graph, &supported_by_graph));
        assert!(is_safe_to_remove(&brick_c, &support_graph, &supported_by_graph));

        let removed = remove_safe_bricks(fallen);
        assert_eq!(removed.len(), 2);
    }
}