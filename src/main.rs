// use freecli::card::{Card, Suit};
use freecli::gamestate::GameState;

fn main() {
    // An initial empty gamestate with random shuffled columns
    let state = GameState::reset();

    println!("{}", state);
}
