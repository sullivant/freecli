use iocraft::prelude::Text;
use serde::{Serialize, Deserialize};
use iocraft::prelude::*;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
    None,
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

    // Returns a iocraft Text element with this card's data
    pub fn as_element(&self) -> Element<'_, iocraft::components::View> {
        element! {
            View() {
                Text(
                    content: self.display_string(),
                    color: match self.suit {
                        Suit::Hearts | Suit::Diamonds => Color::Red,
                        Suit::Clubs | Suit::Spades => Color::Black,
                        Suit::None => Color::Black,
                    }
                )
            }
        }
    }

    pub fn display_string(&self) -> String {
        let rank = match self.rank {
            1 => " A".to_string(),
            11 => " J".to_string(),
            12 => " Q".to_string(),
            13 => " K".to_string(),
            0 => "  ".to_string(),
            n => format!("{:>2}", n).to_string(),
        };
    
        let suit_char = match self.suit {
            Suit::Spades => '\u{2660}',
            Suit::Hearts => '\u{2665}',
            Suit::Diamonds => '\u{2666}',
            Suit::Clubs => '\u{2663}',
            Suit::None => ' '
        };

        let styled = match self.suit {
            Suit::Hearts | Suit::Diamonds => format!("{:>3}{}", rank, suit_char),
            Suit::Clubs | Suit::Spades => format!("{:>3}{}", rank, suit_char),
            Suit::None => format!("{:>3}{}", rank, suit_char),
        };

        styled.to_string()
    }

    // If this card's value is less than the other card and its
    // suit is the opposite in color, we can work with this.
    pub fn can_stack_onto(&self, other: &Card) -> bool {
        let this_red = matches!(self.suit, Suit::Diamonds | Suit::Hearts);

        let other_red = matches!(other.suit, Suit::Diamonds | Suit::Hearts);

        if this_red && other_red {
            return false;
        }

        if self.rank == (other.rank-1) { 
            return true
        }

        false
    }
}