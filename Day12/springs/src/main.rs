use std::collections::HashMap;
use std::fs;

fn main() {
    // Initialize a cache to store previously computed arrangements
    let mut cache = HashMap::new();

    // Load the original spring data from the file and calculate arrangements
    let test_springs = load_data("real_data.txt", false);
    let actual_arrangements = test_springs.iter()
        .map(|(cfg, cond_groups)| find_arrangement_rec(cfg, cond_groups, &mut cache))
        .sum::<usize>();
    println!("Total Arrangements: {}", actual_arrangements);

    // Repeat the process for the expanded (unfolded) spring data
    let expanded_springs = load_data("real_data.txt", true);
    let expanded_arrangements = expanded_springs.iter()
        .map(|(cfg, cond_groups)| find_arrangement_rec(cfg, cond_groups, &mut cache))
        .sum::<usize>();
    println!("Total Arrangements: {}", expanded_arrangements);
}

// Recursive function to find the number of valid arrangements
fn find_arrangement_rec(cfg: &str, cond_groups: &[usize], cache: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    // Base cases: if the configuration or the condition groups are empty
    if cfg.is_empty() {
        return if cond_groups.is_empty() { 1 } else { 0 };
    }
    if cond_groups.is_empty() {
        return if cfg.contains('#') { 0 } else { 1 };
    }

    // Check if the result is already computed and stored in the cache
    let key = (cfg.to_string(), cond_groups.to_vec());
    if let Some(&cached_value) = cache.get(&key) {
        return cached_value;
    }

    let mut count = 0;
    let char0 = cfg.chars().next().unwrap();

    // Recursive case: decide the fate of the first spring based on its state
    if char0 == '.' || char0 == '?' {
        count += find_arrangement_rec(&cfg[1..], cond_groups, cache);
    }
    if char0 == '#' || char0 == '?' {
        if let Some(&bl) = cond_groups.first() {
            if bl <= cfg.len() && !cfg[..bl].contains('.') && (bl == cfg.len() || cfg.chars().nth(bl) != Some('#')) {
                let new_cfg = if bl == cfg.len() { &cfg[bl..] } else { &cfg[bl+1..] };
                count += find_arrangement_rec(new_cfg, &cond_groups[1..], cache);
            }
        }
    }

    // Store the computed result in the cache
    cache.insert(key, count);
    return count;
}

// Function to load data from the file
fn load_data(file: &str, should_unfold: bool) -> Vec<(String, Vec<usize>)> {
    // Read file and process each line based on the should_unfold flag
    fs::read_to_string(file)
        .expect("Failed to read file")
        .lines()
        .map(|line| if should_unfold { unfold(line) } else { line.to_string() })
        .map(|x|split_data(&x))
        .collect()
}

// Function to split each line into spring configuration and condition groups
fn split_data(line: &str) -> (String, Vec<usize>) {
    let mut parts = line.split(' ');
    let cfg = parts.next().unwrap().to_string();
    let cond_groups = parts.next().unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    (cfg, cond_groups)
}

// Function to unfold the condition records for part two of the problem
fn unfold(line: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Invalid row format");
    }

    let (cfg, nums) = (parts[0], parts[1]);
    // Repeat the configuration and numbers with '?' and ',' separators respectively
    let unfolded_cfg = std::iter::repeat(cfg).take(5).collect::<Vec<&str>>().join("?");
    let unfolded_nums = nums.split(',')
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let unfolded_nums = std::iter::repeat(unfolded_nums).take(5).collect::<Vec<String>>().join(",");

    format!("{} {}", unfolded_cfg, unfolded_nums)
}

fn find_arrangement(cfg: &str, cond_groups: &[usize]) -> usize {
    let n = cfg.len();
    let m = cond_groups.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Base case: If both the spring row and condition groups are exhausted, there's 1 valid arrangement
    dp[n][m] = 1;

    for i in (0..n).rev() {
        for j in (0..=m).rev() {
            // If there are no more condition groups to satisfy
            if j == m {
                dp[i][j] = if cfg[i..].chars().all(|c| c == '.' || c == '?') { 1 } else { 0 };
                continue;
            }

            let mut total_arrangements = 0;

            // Case 1: Current spring is operational or unknown
            if cfg.chars().nth(i) != Some('#') {
                total_arrangements += dp[i + 1][j];
            }

            // Case 2: Current spring starts a damaged group
            let remaining_length = n - i;
            if remaining_length >= cond_groups[j] &&
                (remaining_length == cond_groups[j] || cfg.chars().nth(i + cond_groups[j]) != Some('#')) {

                let valid_group = cfg[i..i + cond_groups[j]].chars().all(|c| c == '#' || c == '?');
                if valid_group {
                    total_arrangements += dp[i + cond_groups[j]][j + 1];
                }
            }

            dp[i][j] = total_arrangements;
        }
    }

    dp[0][0]
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
        let dp_r = find_arrangement(&test_spring.0, &test_spring.1);
        assert_eq!(expected_arrangements, dp_r);

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