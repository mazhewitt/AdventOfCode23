use clap::{Arg, App};
use std::collections::HashMap;
use std::fs;
use snow_island_game::island_games::{GameReveal, calculate_possible_games, parse_game_reveals, caclulate_powers_of_min_games};


fn main() {
    let matches = App::new("Game Calculator")
        .version("1.0")
        .author("Mazda Hewitt")
        .about("Calculates possible games from a file based on available stones")
        .arg(Arg::with_name("red")
            .help("Number of red stones available")
            .required(true)
            .index(1))
        .arg(Arg::with_name("green")
            .help("Number of green stones available")
            .required(true)
            .index(2))
        .arg(Arg::with_name("blue")
            .help("Number of blue stones available")
            .required(true)
            .index(3))
        .arg(Arg::with_name("file")
            .help("File containing game descriptions")
            .required(true)
            .index(4))
        .get_matches();

    let red = matches.value_of("red").unwrap().parse::<u32>().expect("Invalid number for red stones");
    let green = matches.value_of("green").unwrap().parse::<u32>().expect("Invalid number for green stones");
    let blue = matches.value_of("blue").unwrap().parse::<u32>().expect("Invalid number for blue stones");
    let filename = matches.value_of("file").unwrap();

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let games = process_file_contents(&contents);

    let available_cubes = HashMap::from([
        ("red".to_string(), red),
        ("green".to_string(), green),
        ("blue".to_string(), blue),
    ]);

    // Output the sum of possible game IDs

    let possible_games = calculate_possible_games(games.clone(), available_cubes.clone());
    // sum possible games
    let sum_possible_games = possible_games.iter().sum::<u32>();

    let sum_min_power_games = caclulate_powers_of_min_games(games.clone());

    println!("{} {}", sum_possible_games, sum_min_power_games.iter().sum::<u32>());
}

fn process_file_contents(contents: &str) -> HashMap<u32, Vec<GameReveal>> {
    let mut games = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            continue; // Skip lines that do not conform to the expected format
        }

        let game_id = parts[0].trim().replace("Game ", "").parse::<u32>().unwrap_or(0);
        let game_reveals_str = parts[1].trim();

        let game_reveals = parse_game_reveals(game_reveals_str.to_string()).unwrap();

        games.insert(game_id, game_reveals);
    }

    games
}