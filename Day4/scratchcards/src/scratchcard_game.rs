use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Debug, Default, Clone)]
pub struct ScratchCard {
    pub scratchcard_number: i32,
    pub your_numbers: Vec<i32>,
    pub winning_numbers: Vec<i32>,
}

impl ScratchCard {
    // Constructor for creating a new ScratchCard
    fn new(scratchcard_number: i32, your_numbers: Vec<i32>, winning_numbers: Vec<i32>) -> Self {

        ScratchCard {
            scratchcard_number,
            your_numbers,
            winning_numbers
        }

    }

    pub fn from_string(line: &str) -> Result<ScratchCard, String> {
        let parts: Vec<&str> = line.split("|").collect();
        if parts.len() != 2 {
            return Err("Invalid format".to_string());
        }

        let left_side = parts[0].trim();
        let right_side = parts[1].trim();

        let left_parts: Vec<&str> = left_side.split(":").collect();
        if left_parts.len() != 2 {
            return Err("Invalid format on the left side of '|'".to_string());
        }

        let scratchcard_number = left_parts[0].trim()
            .split_whitespace().last() // Get the last word in the "Card X" part
            .ok_or("Invalid scratchcard number".to_string())?
            .parse::<i32>()
            .map_err(|_| "Invalid scratchcard number".to_string())?;

        let winning_numbers = left_parts[1].trim().split_whitespace()
            .map(|num| num.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|_| "Invalid winning number".to_string())?;

        let your_numbers = right_side.split_whitespace()
            .map(|num| num.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|_| "Invalid player number".to_string())?;

        Ok(ScratchCard {
            scratchcard_number,
            winning_numbers,
            your_numbers,
        })
    }

    // Method to calculate the points for this scratchcard, given a set of winning numbers
    pub fn calculate_points(&self) -> i32 {
        let mut first_match = true;

        self.your_numbers.iter()
            .filter(|num| self.winning_numbers.contains(num))
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

    pub fn won_cards(&self) -> Vec<i32> {
        let num_matches = count_matches(&&self.your_numbers, &&self.winning_numbers);
        //return a vector which contains numbers starting at the card number pls 1 to the card number plus num matches plus 1
        let mut won_cards: Vec<i32> = Vec::new();
        for i  in self.scratchcard_number+1..self.scratchcard_number + num_matches as i32 + 1 {
            won_cards.push(i as i32);
        }
        won_cards
    }
}


fn count_matches(list1: &Vec<i32>, list2: &Vec<i32>) -> usize {
    let mut count = 0;

    for num1 in list1 {
        for num2 in list2 {
            if num1 == num2 {
                count += 1;
            }
        }
    }

    count
}

pub fn calc_total_won_scratchacrds(original_cards: &HashMap<i32, ScratchCard>) -> i32 {
    let mut total_won_cards = 0;
    let mut cards_to_process: HashMap<i32, i32> = original_cards.keys().map(|&k| (k, 1)).collect();
    let mut next_batch = HashMap::new();

    while !cards_to_process.is_empty() {
        for (&card_number, &count) in &cards_to_process {
            if let Some(card) = original_cards.get(&card_number) {
                total_won_cards += count;

                for won_card_number in card.won_cards() {
                    *next_batch.entry(won_card_number).or_insert(0) += count;
                }
            }
        }

        std::mem::swap(&mut cards_to_process, &mut next_batch);
        next_batch.clear();
    }

    total_won_cards
}




#[cfg(test)]
mod tests {
    use std::collections::hash_map::Iter;
    use super::*;
    #[test]
    fn test_calculate_points() {

        let scratchcard = ScratchCard::from_string("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
        let points = scratchcard.calculate_points();
        assert_eq!(points, 8); // As per the example in your problem statement
    }

    #[test]
    fn test_scratchcard_witn_no_winning_numers() {
        // Winning numbers are on the left side of '|'
        let winning_numbers = vec![87, 83, 26, 28, 32];

        // Scratchcard numbers are on the right side of '|'
        let scratchcard = ScratchCard::from_string("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").unwrap();

        // Calculate the points
        let points = scratchcard.calculate_points();

        // In this case, there are no matching numbers, so points should be 0
        assert_eq!(points, 0, "The points should be 0 as there are no winning numbers on the card.");
    }

    #[test]
    fn test_win_additional_scratchcards() {
        let original_scratchcards: HashMap<i32, ScratchCard> = HashMap::from_iter(vec![
            (1, ScratchCard::from_string("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap()),
            (2, ScratchCard::from_string("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").unwrap()),
            (3, ScratchCard::from_string("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").unwrap()),
            (4, ScratchCard::from_string("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").unwrap()),
            (5, ScratchCard::from_string("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").unwrap()),
            (6, ScratchCard::from_string("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").unwrap())]);
        let expected_total = 30; // Implement this based on the rule

        let total_scratchcards = calc_total_won_scratchacrds(&original_scratchcards);
        assert_eq!(total_scratchcards, expected_total, "The total number of scratchcards should match the expected total.");
    }




    #[test]
    fn test_won_cards() {
        let scratch_card = ScratchCard {
            scratchcard_number: 1,
            your_numbers: vec![83, 86, 6, 31, 17],
            winning_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
        };

        let won_card_numbers = scratch_card.won_cards();

        assert_eq!(won_card_numbers, vec![2i32, 3i32, 4i32, 5i32, 6i32]);
    }



}