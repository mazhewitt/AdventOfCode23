use cucumber::{given, when, World};

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct CubeGameWorld {
    red: u32,
    green: u32,
    blue: u32
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given(expr = "a bag with {int} red cubes, {int} green cubes, {int} blue cubes")]
fn bag_with(world: &mut CubeGameWorld, red: u32, green: u32, blue: u32) {
    world.red = red;
    world.green = green;
    world.blue = blue;
}

#[when(regex = r"^game (\d) reveals (.+)$")]
fn the_game_reveals(world: &mut CubeGameWorld, game: u32, reveals: String){

}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(CubeGameWorld::run("tests/features"));
}