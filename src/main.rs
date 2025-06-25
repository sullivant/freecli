use freecli::card::{Card, Suit};
use freecli::gamestate::GameState;

fn main() {
    // An initial empty gamestate with random shuffled columns
    let state = GameState {
        freecells: [None, None, None, None],
        foundations: [None, None, None, None],
        columns: [
            vec![Card::new(7, Suit::Spades), Card::new(11, Suit::Clubs)],
            vec![],
            vec![Card::new(13, Suit::Diamonds)],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![Card::new(5, Suit::Hearts)],
        ],
    };

    println!("{}", state);
}
