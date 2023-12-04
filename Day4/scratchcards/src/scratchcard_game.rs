
pub struct ScratchCard {
    your_numbers: Vec<i32>,
}

impl ScratchCard {
    // Constructor for creating a new ScratchCard
    fn new(your_numbers: Vec<i32>) -> Self {
        ScratchCard { your_numbers }
    }

    pub fn from_string(numbers_str: &str) -> Result<Self, String> {
        let numbers = numbers_str
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>();

        match numbers {
            Ok(nums) => Ok(ScratchCard::new(nums)),
            Err(e) => Err(format!("Error parsing numbers: {}", e)),
        }
    }

    // Method to calculate the points for this scratchcard, given a set of winning numbers
    pub fn calculate_points(&self, winning_numbers: &[i32]) -> i32 {
        let mut first_match = true;

        self.your_numbers.iter()
            .filter(|num| winning_numbers.contains(num))
            .fold(0, |mut acc, _| {
                if first_match {
                    acc = 1;
                    first_match = false;
                } else {
                    acc *= 2;
                }
                acc
            })
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_points() {
        let winning_numbers = vec![41, 48, 83, 86, 17];

        let scratchcard = ScratchCard::from_string("83 86  6 31 17  9 48 53").unwrap();
        let points = scratchcard.calculate_points(&winning_numbers);
        assert_eq!(points, 8); // As per the example in your problem statement
    }

    #[test]
    fn test_scratchcard_witn_no_winning_numers() {
        // Winning numbers are on the left side of '|'
        let winning_numbers = vec![87, 83, 26, 28, 32];

        // Scratchcard numbers are on the right side of '|'
        let scratchcard = ScratchCard::from_string("88 30 70 12 93 22 82 36").unwrap();

        // Calculate the points
        let points = scratchcard.calculate_points(&winning_numbers);

        // In this case, there are no matching numbers, so points should be 0
        assert_eq!(points, 0, "The points should be 0 as there are no winning numbers on the card.");
    }
}