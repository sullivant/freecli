use crate::card::{Card, Suit};
use console::Style;
use serde::{Serialize, Deserialize};
use std::fmt::{self, Display, Formatter};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::max;



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
        // Header
        for i in 0..8 {
            let styled = Style::new().underlined().bold().apply_to(format!("C{}", i));
            write!(f, " {}  ",styled)?;
        }
        writeln!(f)?;

        let max_height = self.columns.iter().map(|col| col.len()).max().unwrap_or(0);

        // Row by row
        for row in 0..max_height {
            for col in &self.columns {
                if row < col.len() {
                    write!(f, "{:>3}  ", col[row].display_string())?;
                } else {
                    write!(f, "     ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }

}


impl GameState {
    /**
     * Resets gamestate to an empty board with cards dealt into the columns.
     */
    pub fn reset() -> GameState {
        let mut columns: [Vec<Card>; 8] = Default::default();
        let deck = Self::generate_shuffled_deck(); 
        // Walk each of the cards, they come preshuffled, and stuff them into the columns.
        for (i, card) in deck.into_iter().enumerate() {
            columns[i % 8].push(card);
        }

        GameState {
            freecells: [None, None, None, None],
            foundations: [None, None, None, None],
            columns: columns,
        }

    }

    pub fn generate_shuffled_deck() -> Vec<Card> {
        let mut deck = Vec::with_capacity(52);

        for &suit in &[Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds] {
            for rank in 1..=13 {
                deck.push(Card{ rank, suit});
            }
        }
        
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);

        deck
    }
}