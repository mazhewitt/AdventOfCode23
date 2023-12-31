// module definition



use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;

// Define a custom error type for parsing
#[derive(Debug)]
pub enum GameRevealError {
    RegexError(String),
    ParseError(String),
}

// Lazy static for compiling the regex only once
lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+) (\w+)$").expect("Invalid regex");
}


#[derive(Debug, PartialEq, Default, Clone)]
pub struct GameReveal {
    cube_colour: String,
    cube_count: u32
}

impl FromStr for GameReveal {
    type Err = GameRevealError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE.captures(s).ok_or_else(|| GameRevealError::RegexError(s.to_string()))?;

        let cube_count = captures.get(1)
            .and_then(|m| m.as_str().parse::<u32>().ok())
            .ok_or_else(|| GameRevealError::ParseError(s.to_string()))?;

        let cube_colour = captures.get(2)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| GameRevealError::ParseError(s.to_string()))?;

        Ok(GameReveal { cube_colour, cube_count })
    }
}

pub fn parse_game_reveals(input: String) -> Result<Vec<GameReveal>, GameRevealError> {
    let mut game_reveals = Vec::new();

    for part in input.split(';') {
        for game_reveal_str in part.trim().split(',') {
            let game_reveal = game_reveal_str.trim().parse::<GameReveal>()?;
            game_reveals.push(game_reveal);
        }
    }

    Ok(game_reveals)
}


pub fn calculate_possible_games(reveals: HashMap<u32, Vec<GameReveal>>, available_cubes: HashMap<String, u32>) -> Vec<u32> {
    reveals.into_iter()
        .filter(|(_, game_reveals)| is_game_possible(game_reveals.to_vec(), available_cubes.clone()))
        .map(|(game_id, _)| game_id)
        .collect()
}

fn is_game_possible(mut reveals: Vec<GameReveal>, mut available_cubes: HashMap<String, u32>) -> bool {
    if reveals.is_empty() {
        return true;
    }

    let reveal = reveals.pop().unwrap();

    match available_cubes.get_mut(&reveal.cube_colour) {
        Some(available) if *available >= reveal.cube_count => {
            is_game_possible(reveals, available_cubes)
        },
        _ => false,
    }
}

fn calculate_minimum_possible_stones(input: Vec<GameReveal>) -> Vec<GameReveal> {
    let mut stone_counts = HashMap::new();

    for reveal in input {
        let entry = stone_counts.entry(reveal.cube_colour.clone()).or_insert(0);
        if reveal.cube_count > *entry {
            *entry = reveal.cube_count;
        }
    }

    stone_counts.into_iter()
        .map(|(cube_colour, cube_count)| GameReveal { cube_colour, cube_count })
        .collect()
}

fn calculate_power_of_cubes(min_cubes: Vec<GameReveal>) -> u32 {
    let mut power: u32 = 1;
    for cube in min_cubes {
        power *= cube.cube_count;
    }
    power
}

pub fn caclulate_powers_of_min_games(input: HashMap<u32, Vec<GameReveal>>) -> Vec<u32> {
    let mut powers = Vec::new();
    for (_, reveals) in input {
        let min_cubes = calculate_minimum_possible_stones(reveals);
        let power = calculate_power_of_cubes(min_cubes);
        powers.push(power);
    }
    powers
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_game_reveal_from_str() {
        let reveal = GameReveal::from_str("3 blue").unwrap();
        assert_eq!(reveal, GameReveal {
            cube_colour: "blue".to_string(),
            cube_count: 3,
        });
    }

    #[test]
    // test that a game is possible
    fn test_is_game_possible() {
        let reveals = vec![
            GameReveal {
                cube_colour: "red".to_string(),
                cube_count: 1,
            },
            GameReveal {
                cube_colour: "green".to_string(),
                cube_count: 1,
            },
            GameReveal {
                cube_colour: "blue".to_string(),
                cube_count: 1,
            },
        ];
        let available_cubes = HashMap::from([
            ("red".to_string(), 1),
            ("green".to_string(), 3),
            ("blue".to_string(), 1),
        ]);
        assert_eq!(is_game_possible(reveals, available_cubes), true);
    }





    #[test]
    // negative test that a game is not possible
    fn test_is_game_not_possible() {
        let reveals = vec![
            GameReveal {
                cube_colour: "red".to_string(),
                cube_count: 1,
            },
            GameReveal {
                cube_colour: "green".to_string(),
                cube_count: 5,
            },
            GameReveal {
                cube_colour: "blue".to_string(),
                cube_count: 1,
            },
        ];
        let available_cubes = HashMap::from([
            ("red".to_string(), 1),
            ("green".to_string(), 1),
            ("blue".to_string(), 1),
        ]);
        assert_eq!(is_game_possible(reveals, available_cubes), false);
    }



    // a test for calculate_possible_games
    #[test]
    fn test_calculate_possible_games() {
        let mut reveals = HashMap::new();
        reveals.insert(1, vec![
            GameReveal {
                cube_colour: "red".to_string(),
                cube_count: 1,
            },
            GameReveal {
                cube_colour: "green".to_string(),
                cube_count: 1,
            },
            GameReveal {
                cube_colour: "blue".to_string(),
                cube_count: 1,
            },
        ]);
        reveals.insert(2, vec![
            GameReveal {
                cube_colour: "red".to_string(),
                cube_count: 1,
            },
            GameReveal {
                cube_colour: "green".to_string(),
                cube_count: 5,
            },
            GameReveal {
                cube_colour: "blue".to_string(),
                cube_count: 1,
            },
        ]);
        let available_cubes = HashMap::from([
            ("red".to_string(), 1),
            ("green".to_string(), 3),
            ("blue".to_string(), 1),
        ]);
        assert_eq!(calculate_possible_games(reveals, available_cubes), vec![1]);
    }

    #[test]
    fn min_possible_stones_test() {
        let input = vec![
            "3 blue".parse::<GameReveal>().unwrap(),
            "4 red".parse::<GameReveal>().unwrap(),
            "1 red".parse::<GameReveal>().unwrap(),
            "2 green".parse::<GameReveal>().unwrap(),
            "6 blue".parse::<GameReveal>().unwrap(),
            "2 green".parse::<GameReveal>().unwrap(),
        ];
        let mut expected = vec![
            "4 red".parse::<GameReveal>().unwrap(),
            "2 green".parse::<GameReveal>().unwrap(),
            "6 blue".parse::<GameReveal>().unwrap(),
        ];

        // Sorting the vectors for comparison
        let mut result = calculate_minimum_possible_stones(input);
        result.sort_by(|a, b| a.cube_colour.cmp(&b.cube_colour));
        expected.sort_by(|a, b| a.cube_colour.cmp(&b.cube_colour));

        assert_eq!(expected, result);
    }

    #[test]
    fn finds_the_power_of_the_cubes() {
        let input = vec![
            "4 red".parse::<GameReveal>().unwrap(),
            "2 green".parse::<GameReveal>().unwrap(),
            "6 blue".parse::<GameReveal>().unwrap(),
        ];

        let expected: u32 = 4 * 2 * 6;
        assert_eq!(expected, calculate_power_of_cubes(input));
    }




}
