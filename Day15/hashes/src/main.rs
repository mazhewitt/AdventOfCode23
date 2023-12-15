use std::collections::HashMap;
use std::fs;

fn main() {
    let data = load_data("input.txt");
    let sum = data.iter().map(|s| hash(s)).sum::<usize>();
    println!("sum part 1: {}", sum);
    let lens_calc = calculate_lenses(&data);
    println!("lens calculation: {}", lens_calc);
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl Lens {
    fn new(label: String, focal_length: usize) -> Lens {
        Lens { label, focal_length }
    }
}

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |mut acc, c| {
            acc += c as usize;
            acc *= 17;
            acc %= 256;
            acc
        })
}

fn calculate_lenses(data: &[String]) -> usize {
    let mut hashmap: HashMap<usize, Vec<Lens>> = HashMap::new();
    for lens_op in data {
        process_lens_op(lens_op, &mut hashmap);
    }
    calculate_result(&hashmap)
}

fn process_lens_op(lens_op: &str, hashmap: &mut HashMap<usize, Vec<Lens>>) {
    let parts: Vec<&str> = lens_op.split('=').collect();
    if lens_op.contains('=') && parts.len() == 2 {
        process_assignment(parts, hashmap);
    } else if lens_op.contains('-') {
        process_removal(lens_op, hashmap);
    }
}

fn process_assignment(parts: Vec<&str>, hashmap: &mut HashMap<usize, Vec<Lens>>) {
    let index = hash(parts[0]);
    let index_str = parts[0].to_string();
    let value: usize = parts[1].parse().unwrap();
    let lenses = hashmap.entry(index).or_insert_with(Vec::new);
    update_or_insert_lens(lenses, index_str, value);
}

fn process_removal(lens_op: &str, hashmap: &mut HashMap<usize, Vec<Lens>>) {
    let parts: Vec<&str> = lens_op.split('-').collect();
    if parts.len() == 2 {
        let index = hash(parts[0]);
        let index_str = parts[0].to_string();
        if let Some(lenses) = hashmap.get_mut(&index) {
            remove_lens_if_exists(lenses, &index_str);
        }
    }
}

fn update_or_insert_lens(lenses: &mut Vec<Lens>, label: String, focal_length: usize) {
    if let Some(lens) = lenses.iter_mut().find(|lens| lens.label == label) {
        lens.focal_length = focal_length;
    } else {
        lenses.push(Lens::new(label, focal_length));
    }
}

fn remove_lens_if_exists(lenses: &mut Vec<Lens>, label: &str) {
    if let Some(pos) = lenses.iter().position(|lens| lens.label == label) {
        lenses.remove(pos);
    }
}

fn calculate_result(hashmap: &HashMap<usize, Vec<Lens>>) -> usize {
    hashmap.iter().fold(0, |acc, (box_n, lenses)| {
        acc + lenses
            .iter()
            .enumerate()
            .fold(0, |acc, (index, lens)| acc + (box_n + 1) * (index + 1) * lens.focal_length)
    })
}

fn load_data(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .expect("Something went wrong reading the file")
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_load_data() {
        let data: Vec<String> = load_data("test.txt");
        assert_eq!(data.len(), 11); // Adjust the expected length according to your test file content
    }

    #[test]
    fn test_calculate_lenses() {
        let data: Vec<String> = load_data("test.txt");
        assert_eq!(calculate_lenses(&data), 145);
    }
}
