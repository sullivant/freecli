use console::Style;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    pub rank: u8, // 1 - 13
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn display_string(&self) -> String {
        let rank = match self.rank {
            1 => " A".to_string(),
            11 => " J".to_string(),
            12 => " Q".to_string(),
            13 => " K".to_string(),
            n => format!("{:>2}", n).to_string(),
        };
    
        let suit_char = match self.suit {
            Suit::Spades => '\u{2660}',
            Suit::Hearts => '\u{2665}',
            Suit::Diamonds => '\u{2666}',
            Suit::Clubs => '\u{2663}'
        };

        let styled = match self.suit {
            Suit::Hearts | Suit::Diamonds => Style::new().red().apply_to(format!("{:>2}{}", rank, suit_char)),
            Suit::Clubs | Suit::Spades => Style::new().apply_to(format!("{}{}", rank, suit_char)),
        };

        styled.to_string()
    }
}