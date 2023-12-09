fn main() {
    let data = load_data(&"input_file.txt".to_string());
    let total:i32 = data.iter().map(|row| {
        let reversed = row.iter().rev().map(|&x| x).collect::<Vec<i32>>();
        generate_next_value(reversed)
    }).sum();
    println!("Total: {}", total);
}

// load the test data
fn load_data(filename:&String) -> Vec<Vec<i32>> {
    let file = std::fs::read_to_string(filename).unwrap();
    let lines = file.lines();
    let mut data = vec![];
    for line in lines {
        let mut row = vec![];
        for num in line.split(" ") {
            row.push(num.parse::<i32>().unwrap());
        }
        data.push(row);
    }
    data
}

fn generate_differences(input: &Vec<i32>) -> Vec<i32> {
    // base case
    if input.len() == 1{
        return vec![];
    }
    let first_diff = input[1] - input[0];
    let mut rest_sequence = input[1..].to_vec();
    let mut rest_diff = generate_differences(&rest_sequence);
    rest_diff.insert(0, first_diff);

    rest_diff
}

fn generate_next_value(sequence: Vec<i32>) -> i32 {
    let mut sequences = vec![sequence];
    while !sequences.last().unwrap().iter().all(|&x| x == 0) {
        let last_sequence = sequences.last().unwrap().to_vec();
        sequences.push(generate_differences(&last_sequence));
    }

    let mut next_value = 0;
    for seq in sequences.iter().rev() {
        if !seq.is_empty() {
            next_value += seq.last().unwrap();
        }
    }

    next_value
}


#[test]
fn test_generate_differences() {
    let sequence = vec![0, 3, 6, 9, 12, 15];
    let expected = vec![3, 3, 3, 3, 3];
    assert_eq!(generate_differences(&sequence), expected);
}

#[test]
fn test_generate_next_value() {
    let sequence = vec![0, 3, 6, 9, 12, 15];
    assert_eq!(generate_next_value(sequence), 18);
}
// test this case vec![10,  13,  16,  21,  30,  45]
#[test]
fn test_generate_next_value2() {
    let sequence = vec![10,  13,  16,  21,  30,  45];
    assert_eq!(generate_next_value(sequence), 68);
}




