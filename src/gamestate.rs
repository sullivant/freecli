use crate::card::{Card, Suit};
use crate::moves::{LocationType, Move};
use console::Style;
use serde::{Serialize, Deserialize};
use std::fmt::{self, Display, Formatter};
use rand::seq::SliceRandom;
use rand::thread_rng;


/**
 * Handles various aspects of the current and potential future game states, including:
 *  - Printing of the state to the CLI
 *  - Checking to see if an attempted move is valid
 *  - Reset of the game state 
 *  - Applying the move if it is valid
 *  - Checking for a winning condition
 */


#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub freecells: [Option<Card>; 4],
    pub foundations: [Option<Card>; 4],
    pub columns: [Vec<Card>; 8],
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Freecells:   ")?;
        for cell in &self.freecells {
            match cell {
                Some(card) => write!(f, "[{}] ", card.display_string())?,
                None => write!(f, "[   ] ")?,
            }
        }

        write!(f, "Foundations:   ")?;
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
            columns,
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

    /// Checks if the game stat is in a winning condition, returning TRUE if the game is won.
    /// 
    /// Stat for a win is:
    ///     All columns are empty ("empty")
    ///     All freecells are empty ("none")
    /// 
    /// Since we do not allow discards, foundation is therefore not necessary to check.
    /// 
    pub fn is_win(&self) -> bool {
        self.columns.iter().all(|col| col.is_empty()) &&
        self.freecells.iter().all(|cell| cell.is_none())
    }

    /// Does the actually checking of a move to see if it is valid
    /// 
    pub fn check_move(&mut self, mv: &Move) -> Result<(), String> {
        // - Can only move from a non-empty location
        // - Can only move onto a stack where the color is different and the rank is higher
        //      (ie: red 2 can stack onto a black 3)
        // - Foundations must go up in order
        // Freecells must be empty

        match (mv.from, mv.to) {
            // Moving from a col to a col
            (LocationType::Column, LocationType::Column) => {
                let src = self.columns.get(mv.from_idx).ok_or("Invalid source column.")?;
                let dst = self.columns.get(mv.to_idx).ok_or("Invalid destination column.")?;

                let card = src.last().ok_or("Source column is empty.")?;
                if let Some(top_dst) = dst.last() {
                    if !card.can_stack_onto(top_dst) {
                        return Err("Illegal move, cannot stack.".into());
                    }
                }
                Ok(())
            },

            // From a column to a freecell
            (LocationType::Column, LocationType::Freecell) => {
                let cell = self.freecells.get(mv.to_idx).ok_or("Invalid freecell index.")?;
                if cell.is_some() {
                    return Err("Freecell is occupied!".to_string());
                }
                let src = self.columns.get(mv.from_idx).ok_or("Invalid source column.")?;
                if src.is_empty() {
                    return Err("Column is empty.".to_string());
                }
                Ok(())
            },

            // Freecell to a column
            (LocationType::Freecell, LocationType::Column) => {
                let card = self.freecells.get(mv.from_idx).ok_or("Invalid freecell")?;
                let dst = self.columns.get(mv.to_idx).ok_or("Invalid destination column.")?;
                if let Some(top_dst) = dst.last() {
                    match card {
                        Some(c) => {
                            if !c.can_stack_onto(top_dst) {
                               return Err("Illegal move, cannot stack.".into());
                            }
                        },
                        _ => {return Err("Freecell is empty.".into());}
                    }                    
                }
                Ok(())
            },

            (LocationType::Column, LocationType::Foundation) | (LocationType::Freecell, LocationType::Foundation) => {
                let src = self.columns.get(mv.from_idx).ok_or("Invalid source column.")?;
                let card: &Card = match mv.from {
                    LocationType::Column => {
                        src.last().ok_or("Source column is empty.")?
                    },
                    LocationType::Freecell => {
                        &self.freecells.get_mut(mv.from_idx)
                            .ok_or("Invalid freecell index.")?
                            .take()
                            .ok_or("Freecell is empty.")?
                    },
                    _ => Err("Invalid source location for a move to foundation")?
                };
                
                let index = match card.suit {
                    Suit::Spades => 0,
                    Suit::Hearts => 1,
                    Suit::Diamonds => 2,
                    Suit::Clubs => 3
                };

                let foundation = &mut self.foundations[index];

                // Attempt the actual move here, the actual move will be done later.
                match foundation {
                    Some(top) if card.rank == top.rank + 1 => {
                        Ok(())
                    },
                    None if card.rank == 1 => {
                        Ok(())
                    },
                    _ => Err("Invalid foundation move!".into()),
                    // TODO: When invalid move, place it back into the source location!
                }
            },
            _ => Err("Unsupported move type".into())
        }
    }

    pub fn apply_move(&mut self, mv: Move) -> Result<(), String> {
        self.check_move(&mv)?; // If a move is not valid, we'll bubble up the error with a reason.
        match (mv.from, mv.to) {
            (LocationType::Column, LocationType::Column) => {
                // Col to Col move.
                let card = self.columns.get_mut(mv.from_idx)
                    .ok_or("Invalid source column")?.pop()
                    .ok_or("Source column is empty.")?;

                self.columns.get_mut(mv.to_idx)
                    .ok_or("Target column invalid.")?
                    .push(card);

                Ok(())
            },
            (LocationType::Column, LocationType::Freecell) => {
                let card = self.columns.get_mut(mv.from_idx)
                    .ok_or("Invalid source column")?.pop()
                    .ok_or("Source column is empty.")?;

                let cell = self.freecells.get_mut(mv.to_idx)
                    .ok_or("Invalid target freecell.")?;

                if cell.is_some() {
                    return Err("Freecell is already occupied.".into());
                }

                *cell = Some(card);

                Ok(())
            },
            (LocationType::Freecell, LocationType::Column) => {
                let card = self.freecells.get_mut(mv.from_idx)
                    .ok_or("Invalid freecell index.")?
                    .take()
                    .ok_or("Freecell is empty.")?;

                self.columns.get_mut(mv.to_idx)
                    .ok_or("Invalid destination column")?
                    .push(card);

                Ok(())
            },
            (LocationType::Column, LocationType::Foundation) => {
                let card = self.columns.get_mut(mv.from_idx)
                    .ok_or("Invalid source column")?.pop()
                    .ok_or("Source column is empty.")?;

                self.place_in_foundation(card)
            },
            (LocationType::Freecell, LocationType::Foundation) => {
                let card = self.freecells.get_mut(mv.from_idx)
                    .ok_or("Invalid freecell index.")?
                    .take()
                    .ok_or("Freecell is empty.")?;

                self.place_in_foundation(card)
            },

            _ => Err("Unsupported move combination".into()),
        }

    }

    fn place_in_foundation(&mut self, card: Card) -> Result<(), String> {
        let index = match card.suit {
            Suit::Spades => 0,
            Suit::Hearts => 1,
            Suit::Diamonds => 2,
            Suit::Clubs => 3
        };

        let foundation = &mut self.foundations[index];

        match foundation {
            Some(top) if card.rank == top.rank + 1 => {
                *foundation = Some(card); // Do the actual place if it can apply here
                Ok(())
            },
            None if card.rank == 1 => {
                *foundation = Some(card);
                Ok(())
            },
            _ => Err("Invalid foundation move!".into()),
            // TODO: When invalid move, place it back into the source location!
        }
    }
}