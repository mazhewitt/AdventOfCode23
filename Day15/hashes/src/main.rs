use std::fs;

fn main() {
    let data = load_data("input.txt");
    let sum =  data.iter().map(|s| hash(s)).sum::<usize>();
    println!("sum: {}", sum);
}
/*
The HASH algorithm is a way to turn any string of characters into a single number in the range 0 to 255. To run the HASH algorithm on a string, start with a current value of 0. Then, for each character in the string starting from the beginning:

Determine the ASCII code for the current character of the string.
Increase the current value by the ASCII code you just determined.
Set the current value to itself multiplied by 17.
Set the current value to the remainder of dividing itself by 256.
After following these steps for each character in the string in order, the current value is the output of the HASH algorithm.
 */

fn hash(s: &str) -> usize {
    let mut current_value = 0;
    for c in s.chars() {
        let ascii_code = c as usize;
        current_value += ascii_code;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn load_data(file: &str) -> Vec<String> {
    //load file into string
    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    // split string into vector of strings by comma
    let data: Vec<String> = contents.split(",")
        .map(|s| s.to_string())
        .collect();
    data

}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"),52);
    }
    #[test]
    fn test_load_data() {
        let data: Vec<String> = load_data("test.txt");
        assert_eq!(data.len(), 11);
    }

    #[test]

    fn can_build_map(){
        let data: Vec<String> = load_data("test.txt");
        let hashmap : HashMap<usize, HashMap<String, usize>>;
        for lens_op in data{
            if lens_op.contains(r"="){
                println!("add_op: {}", lens_op);
                let parts:Vec<Option(String)> = lens_op.split(r"=").collect();
                let index_s:String = Some(parts[0]).to_string().unwrap();
                let index = hash(&index_s);
                let value:usize = Some(parts[1]).to_string().unwrap().parse();
                let maybe_lenses = hashmap.get(&index);
                match maybe_lenses{
                    Some(mut lenses) =>{
                        lenses.add(index_s, value)
                    }
                    _ =>{
                        let lenses:HashMap<String, usize> = HashMap::new();
                        lenses.add(index_s, value);
                        hashmap.add(index, lenses);
                    }
                }
            }
            else if lens_op.contains(r"-"){
                println!("removeop_op: {}", lens_op);
                let parts = lens_op.split(r"-").collect();
                let index_s:String = Some(parts[0]).to_string().unwrap();
                let index = hash(&index_s);
                let value:usize = Some(parts[1]).to_string().unwrap().parse();
                let maybe_lenses = hashmap.get(&index);
                match maybe_lenses{
                    Some (mut lenses) =>{
                        lenses.remove(&index_s)
                    }
                    _ => {}
                }
            }
        }
    }

}