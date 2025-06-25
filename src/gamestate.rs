use crate::card::Card;
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
