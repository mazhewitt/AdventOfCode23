use std::{fs};
use std::collections::HashMap;

fn main() {
    let cache = &mut HashMap::new();
    let test_springs = load_data("real_data.txt", false);
    let actual_arrangements = test_springs.iter().map(|(cfg, cond_groups)| find_arrangement_rec(cfg, cond_groups, cache)).sum::<usize>();
    println!("Total Arrangements: {}", actual_arrangements);
    let expanded_springs = load_data("real_data.txt", true);
    let expanded_arrangements = expanded_springs.iter().map(|(cfg, cond_groups)| find_arrangement_rec(cfg, cond_groups, cache)).sum::<usize>();
    println!("Total Arrangements: {}", expanded_arrangements);

}



fn find_arrangement_rec(cfg: &str, cond_groups: &Vec<usize>, cache: &mut HashMap<(String, Vec<usize>),usize>) -> usize {
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
    if (cache.contains_key(&(cfg.to_string(), cond_groups.to_vec()))) {
        return *cache.get(&(cfg.to_string(), cond_groups.to_vec())).unwrap();
    }

    let mut count = 0;

    let char0 = cfg.chars().nth(0).unwrap();

    if (char0 == '.') | (char0 == '?') {
        count += find_arrangement_rec(&cfg[1..], cond_groups, cache);
    }

    if (char0 == '#') | (char0 == '?') {
        // we are in a block
        let bl = cond_groups[0];

        if bl  <= cfg.len() && !&cfg[0..bl].contains(".") && (bl == cfg.len() || cfg.chars().nth(bl).unwrap() != '#') {
            if bl == cfg.len(){
                count += find_arrangement_rec(&cfg[bl..], &cond_groups[1..].to_vec(), cache);
            }
            else {
                count += find_arrangement_rec(&cfg[bl + 1..], &cond_groups[1..].to_vec(), cache);
            }
        }
    }
    cache.insert((cfg.to_string(), cond_groups.to_vec()), count);
    count
}

fn load_data(file: &str, should_unfold:bool) -> Vec<(String, Vec<usize>)> {

    fs::read_to_string(file)
        .expect("Failed to read file")
        .lines()
        .map(|line| if should_unfold { unfold(line) } else { line.to_string() })
        .map(|line| split_data(&line))
        .collect()

}

fn split_data(line: &str) -> (String, Vec<usize>) {
    let mut parts = line.split(" ");
    let cfg = parts.next().unwrap().to_string();
    let cond_groups = parts.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    (cfg, cond_groups)
}

fn unfold(line: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Invalid row format");
    }

    let (cfg, nums) = (parts[0], parts[1]);

    let unfolded_cfg = std::iter::repeat(cfg).take(5).collect::<Vec<&str>>().join("?");

    let nums: Vec<String> = nums.split(',')
        .map(|num| num.to_string())
        .collect();
    let unfolded_nums = nums.join(",");
    let unfolded_nums = std::iter::repeat(unfolded_nums).take(5).collect::<Vec<String>>().join(",");

    format!("{} {}", unfolded_cfg, unfolded_nums)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_load_data() {
        let file = "test_data.txt";
        let data = load_data(file, false);
        assert_eq!(data.len(), 6);
    }

    #[test]
    fn test_can_find_arrangements_for_single_case() {
        let test_spring = split_data("?###???????? 3,2,1");
        let expected_arrangements = 10;
        let cache = &mut HashMap::new();
        let actual_arrangements = find_arrangement_rec(&test_spring.0, &test_spring.1, cache);
        assert_eq!(expected_arrangements, actual_arrangements);

    }

    #[test]
    fn test_can_find_arrangements_for_multiple_cases() {
        let test_springs = load_data("test_data.txt", false);
        let cache = &mut HashMap::new();
        let actual_arrangements = test_springs.iter().map(|(cfg, cond_groups)| find_arrangement_rec(cfg, cond_groups, cache)).sum::<usize>();
        assert_eq!(21, actual_arrangements);

    }

    #[test]
    fn test_unfold() {
        let input = ".# 1";
        let expected = ".#?.#?.#?.#?.# 1,1,1,1,1";
        assert_eq!(unfold(input), expected);

        let input2 = "???.### 1,1,3";
        let expected2 = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";
        assert_eq!(unfold(input2), expected2);
    }

    #[test]
    fn test_can_find_arrangements_for_expanded_case() {
        let test_spring = split_data(&unfold("?###???????? 3,2,1"));
        let expected_arrangements = 506250;
        let cache = &mut HashMap::new();
        let actual_arrangements = find_arrangement_rec(&test_spring.0, &test_spring.1, cache);
        assert_eq!(expected_arrangements, actual_arrangements);

    }

}