use cucumber::{given, then, when, World};
use std::collections::HashMap;
use std::str::FromStr;
use snow_island_game::island_games::{GameReveal, calculate_possible_games};


// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct CubeGameWorld {
    available_cubes: HashMap<String, u32>,
    game_reveals: HashMap<u32, Vec<GameReveal>>
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given(expr = "a bag with {int} red cubes, {int} green cubes, {int} blue cubes")]
fn bag_with(world: &mut CubeGameWorld, red: u32, green: u32, blue: u32) {
    world.available_cubes.insert("red".to_string(), red);
    world.available_cubes.insert("green".to_string(), green);
    world.available_cubes.insert("blue".to_string(), blue);
}

#[when(regex = r"^Game (\d): (.+)$")]
fn the_game_reveals(world: &mut CubeGameWorld, game: u32, reveals: String){
    let game_reveals = reveals.split(", ")
        .map(|reveal| GameReveal::from_str(reveal).unwrap())
        .collect::<Vec<GameReveal>>();

    world.game_reveals.insert(game, game_reveals);

}

#[then(regex = r"^the possible games are (.*) and the sum is (\d+)$")]
fn calc_possible_games (world: &mut CubeGameWorld, possible_games: String, sum: u32){
    let possible_games = calculate_possible_games(world.game_reveals.clone(), world.available_cubes.clone());
    // sum possible games
    let sum_possible_games = possible_games.iter().sum::<u32>();
    // assert that the sum is correct
    assert_eq!(sum_possible_games, sum);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(CubeGameWorld::run("tests/features"));
}