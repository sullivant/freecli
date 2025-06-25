use freecli::card::{Card, Suit};
use freecli::gamestate::GameState;

fn main() {
    let state = GameState {
        freecells: [None, Some(card(1, Suit::Hearts)), None, Some(card(4, Suit::Clubs))],
        foundations: [Some(card(5, Suit::Clubs)), None, None, None],
        columns: [
            vec![card(7, Suit::Spades), card(11, Suit::Clubs)],
            vec![],
            vec![card(13, Suit::Diamonds)],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![card(5, Suit::Hearts)],
        ],
    };

    println!("{}", state);
}

fn card(rank: u8, suit: Suit) -> Card {
    Card { rank, suit }
}