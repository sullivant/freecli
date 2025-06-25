use crate::card::{Card, Suit};
use serde::{Serialize, Deserialize};
use std::fmt::{self, Display, Formatter};



#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub freecells: [Option<Card>; 4],
    pub foundations: [Option<Card>; 4],
    pub columns: [Vec<Card>; 8],
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Freecells:   ")?;
        for cell in &self.freecells {
            match cell {
                Some(card) => write!(f, "[{}] ", card.display_string())?,
                None => write!(f, "[   ] ")?,
            }
        }
        writeln!(f)?;

        writeln!(f, "Foundations:   ")?;
        for cell in &self.foundations {
            match cell {
                Some(card) => write!(f, "[{}] ", card.display_string())?,
                None => write!(f, "[   ] ")?,
            }
        }

        writeln!(f, "\n\nColumns:")?;
        for (i, col) in self.columns.iter().enumerate() {
            write!(f, "C{}: ", i)?;
            for card in col {
                write!(f, "{} ", card.display_string())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }

}


impl GameState {
    pub fn reset(&self) -> GameState {
        GameState {
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
    }
    }
}