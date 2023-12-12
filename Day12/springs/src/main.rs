use std::{fs};

fn main() {
    let test_springs = load_data("real_data.txt");
    let actual_arrangements = test_springs.iter().map(|(cfg, cond_groups)| find_arrangement_rec(cfg, cond_groups)).sum::<usize>();
    println!("Total Arrangements: {}", actual_arrangements);
}



fn find_arrangement_rec(cfg: &str, cond_groups: &Vec<usize>) -> usize {
    if cfg.len() == 0 {
        return if cond_groups.len() == 0 {
            1
        } else {
            0
        }
    }

    if cond_groups.len() == 0 {
        return if cfg.contains("#")
        {
            0
        } else {
            1
        }
    }
    let mut count = 0;

    let char0 = cfg.chars().nth(0).unwrap();

    if (char0 == '.') | (char0 == '?') {
        count += find_arrangement_rec(&cfg[1..], cond_groups);
    }

    if (char0 == '#') | (char0 == '?') {
        // we are in a block
        let bl = cond_groups[0];

        if bl  <= cfg.len() && !&cfg[0..bl].contains(".") && (bl == cfg.len() || cfg.chars().nth(bl).unwrap() != '#') {
            if bl == cfg.len(){
                count += find_arrangement_rec(&cfg[bl..], &cond_groups[1..].to_vec());
            }
            else {
                count += find_arrangement_rec(&cfg[bl + 1..], &cond_groups[1..].to_vec());
            }
        }
    }

    count
}

fn load_data(file: &str) -> Vec<(String, Vec<usize>)> {

    fs::read_to_string(file)
        .expect("Failed to read file")
        .lines()
        .map(|line| split_data(line))
        .collect()

}

fn split_data(line: &str) -> (String, Vec<usize>) {
    let mut parts = line.split(" ");
    let cfg = parts.next().unwrap().to_string();
    let cond_groups = parts.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    (cfg, cond_groups)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_load_data() {
        let file = "test_data.txt";
        let data = load_data(file);
        assert_eq!(data.len(), 6);
    }

    #[test]
    fn test_can_find_arrangements_for_single_case() {
        let test_spring = split_data("?###???????? 3,2,1");
        let expected_arrangements = 10;
        let actual_arrangements = find_arrangement_rec(&test_spring.0, &test_spring.1);
        assert_eq!(expected_arrangements, actual_arrangements);

    }

    #[test]
    fn test_can_find_arrangements_for_multiple_cases() {
        let test_springs = load_data("test_data.txt");
        let actual_arrangements = test_springs.iter().map(|(cfg, cond_groups)| find_arrangement_rec(cfg, cond_groups)).sum::<usize>();
        assert_eq!(21, actual_arrangements);

    }

}