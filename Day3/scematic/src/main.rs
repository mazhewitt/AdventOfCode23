fn main() {
    println!("Hello, world!");
}

//Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_character_grid() {
        // a two dimensional array of characters


        let grid:&[&[_]] = load_character_grid("test_file.txt");

    }
}