use std::cmp::Ordering;
use std::fs;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    hand_type: HandType,
    score: u32, // New score field with a default value of 0
}

impl Hand {
    fn new(cards: String) -> Hand {
        let hand_type = Hand::determine_hand_type(&cards);
        Hand { cards, hand_type, score: 0 }
    }

    fn new_with_score(cards: String, score: u32) -> Hand {
        let hand_type = Hand::determine_hand_type(&cards);
        Hand { cards, hand_type, score }
    }

    fn determine_hand_type(hand: &String) -> HandType {
        let mut card_counts = std::collections::HashMap::new();

        // Count the occurrences of each card
        for card in hand.chars() {
            *card_counts.entry(card).or_insert(0) += 1;
        }

        let mut counts: Vec<_> = card_counts.values().cloned().collect();
        counts.sort(); // Sort the counts

        match counts.as_slice() {
            [5, ..] => HandType::FiveOfAKind,
            [1, 4, ..] | [4, 1, ..] => HandType::FourOfAKind,
            [2, 3, ..] | [3, 2, ..] => HandType::FullHouse,
            [1, 1, 3, ..] | [1, 3, 1, ..] | [3, 1, 1, ..] => HandType::ThreeOfAKind,
            [1, 2, 2, ..] | [2, 1, 2, ..] | [2, 2, 1, ..] => HandType::TwoPair,
            [1, 1, 1, 2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
    fn compare_cards(&self, other: &Self) -> Ordering {
        // Assert that hand types are the same
        assert_eq!(self.hand_type, other.hand_type, "compare_cards called with hands of different types");

        let card_value = |card: char| -> u8 {
            match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '2'..='9' => card.to_digit(10).unwrap() as u8,
                _ => 0, // Default case for unexpected characters
            }
        };

        // loop though the cards in the hands with zip
        for (card1, card2) in self.cards.chars().zip(other.cards.chars()) {

            let card1_value = card_value(card1);
            let card2_value = card_value(card2);

            if card1_value != card2_value {
                let ordering =  card1_value.cmp(&card2_value);
                return ordering;
            }

        }
        Ordering::Equal
    }


}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            self.compare_cards(other)
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type // && other comparison logic if needed
    }
}

impl Eq for Hand {}

fn load_data_frm_file(filename: &str) -> Vec<Hand> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let hands: Vec<Hand> = contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let cards = parts[0].to_string();
            let score = parts[1].parse::<u32>().expect("Invalid score format");
            Hand::new_with_score(cards, score)
        })
        .collect();
    hands
}

fn calculate_total_winnings(hands: &mut [Hand]) -> u32 {
    // Sort the hands by strength (the Ord implementation should handle this)
    // Sort in descending order of strength, so the strongest hand gets the highest rank
    hands.sort();

    // Calculate the total winnings
    // Enumerate provides the index, which is one less than the rank (since index starts at 0)
    hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        let rank = index + 1; // Rank starts from 1, not 0
        acc + (hand.score * rank as u32) // Calculate the winnings for this hand and add to the total
    })
}




    #[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_loading_hands_from_file() {
        let filename = "test_data.txt";

        let hands = load_data_frm_file(filename);

        // Example assertion
        assert_eq!(hands[0], Hand::new_with_score(String::from("32T3K"), 765));
    }


    #[test]
    fn test_hand_ordering_and_winnings_calculation() {
        let mut hands = vec![
            Hand::new_with_score(String::from("32T3K"), 765), // One pair
            Hand::new_with_score(String::from("KK677"), 28),  // Two pair

            Hand::new_with_score(String::from("T55J5"), 684), // Three of a kind
            Hand::new_with_score(String::from("QQQJA"), 483), // Three of a kind
            Hand::new_with_score(String::from("KTJJT"), 220), // Two pair
        ];



        // Assert the order of hands
        let expected_order = vec!["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"];
        for (hand, &expected_cards) in hands.iter().zip(expected_order.iter()) {
            println!("Hand: {:?} HandType {:?}", hand.cards, hand.hand_type);
        }

        let total_winnings = calculate_total_winnings(&mut hands);

        // Assert the total winnings
        assert_eq!(total_winnings, 6440);
    }

        #[test]
    fn test_hand_types() {
        let hands = vec![
            (Hand::new(String::from("32T3K")), HandType::OnePair), // One pair
            (Hand::new(String::from("KK677") ), HandType::TwoPair), // Two pair
            (Hand::new(String::from("T55J5") ), HandType::ThreeOfAKind), // Three of a kind
            (Hand::new(String::from("23332") ), HandType::FullHouse), // Full house
            (Hand::new(String::from("32323") ), HandType::FullHouse), // Full house
            (Hand::new(String::from("AA8AA") ), HandType::FourOfAKind), // Four of a kind
            (Hand::new(String::from("AAAAA") ), HandType::FiveOfAKind), // Five of a kind
            (Hand::new(String::from("23456") ), HandType::HighCard), // High card
        ];

        for (hand, expected_type) in hands {
            assert_eq!(hand.hand_type, expected_type, "Failed at hand: {:?}", hand);
        }
    }

        #[test]
        fn test_hand_ordering() {
            let mut hands = vec![
                Hand::new(String::from("32T3K")), // One pair
                Hand::new(String::from("KK677")), // Two pair
                Hand::new(String::from("AAAAA")), // Five of a kind
                Hand::new(String::from("T55J5")), // Three of a kind
                Hand::new(String::from("23332")), // Full house
                Hand::new(String::from("AA8AA")), // Four of a kind
                Hand::new(String::from("23456")), // High card
            ];

            // Sort the hands based on their ordering
            hands.sort();

            // Expected order (from weakest to strongest)
            let expected_order = vec![
                Hand::new(String::from("23456")), // High card
                Hand::new(String::from("32T3K")), // One pair
                Hand::new(String::from("KK677")), // Two pair
                Hand::new(String::from("T55J5")), // Three of a kind
                Hand::new(String::from("23332")), // Full house
                Hand::new(String::from("AA8AA")), // Four of a kind
                Hand::new(String::from("AAAAA")), // Five of a kind
            ];

            // Compare the order of hands against the expected order
            for (hand, expected_hand) in hands.iter().zip(expected_order.iter()) {
                assert_eq!(hand, expected_hand, "Failed at hand: {:?}", hand);
            }
        }

        #[test]
        fn test_secondary_sorting_rules() {
            let mut hands = vec![
                Hand::new(String::from("33332")), // Four of a kind
                Hand::new(String::from("2AAAA")), // Four of a kind
                Hand::new(String::from("77888")), // Full house
                Hand::new(String::from("77788")), // Full house
            ];

            // Sort the hands based on their ordering
            hands.sort();

            // Expected order (based on secondary sorting rules)
            let expected_order = vec![
                Hand::new(String::from("77888")), // Full house, stronger third card
                Hand::new(String::from("77788")), // Full house, weaker third card
                Hand::new(String::from("33332")), // Four of a kind, stronger first card
                Hand::new(String::from("2AAAA")), // Four of a kind, weaker first card
            ];

            // Compare the order of hands against the expected order
            for (hand, expected_hand) in hands.iter().zip(expected_order.iter()) {
                assert_eq!(hand, expected_hand, "Failed at hand: {:?}", hand);
            }
        }

}




fn main() {
    //load real data
    let mut hands = load_data_frm_file("camel_card_data.txt");
    let total_winnings = calculate_total_winnings(&mut hands);
    // print the total winnings
    println!("Total winnings: {}", total_winnings);

}
