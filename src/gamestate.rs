use crate::card::{Card, Suit};
use crate::moves::{LocationType, Move};
// use console::Style;
use rand::{RngCore, SeedableRng};
use serde::{Serialize, Deserialize};
use std::fmt::{self, Display, Formatter};
use rand::seq::SliceRandom;
use rand::rngs::{OsRng, StdRng}; // For the seed randomization

use iocraft::prelude::*;

/**
 * Handles various aspects of the current and potential future game states, including:
 *  - Printing of the state to the CLI
 *  - Checking to see if an attempted move is valid
 *  - Reset of the game state 
 *  - Applying the move if it is valid
 *  - Checking for a winning condition
 */


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub freecells: [Option<Card>; 4],
    pub foundations: [Option<Card>; 4],
    pub columns: [Vec<Card>; 8],
    pub history: Vec<Move>,
    pub seed: u64,
    pub last_move_error: Option<String>,
}


/// Convenience fn to determine the ordinal index for a given card's suit.
fn get_foundation_index(card: Card) -> usize {
    match card.suit {
        Suit::Spades => 0,
        Suit::Hearts => 1,
        Suit::Diamonds => 2,
        Suit::Clubs => 3,
        _ => 0,
    }
}


impl Display for GameState {
    /// The display of the main game state and its formatting over various lines on the CLI
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {

        // For the display of our last move
        let last_move_color = if self.last_move_error.is_some() {Color::Red} else {Color::White};
        let last_move_border = if self.last_move_error.is_some() {BorderStyle::Double} else {BorderStyle::Single};
        

        // Vector preparation for freecells, foundation, and tableau
        let foundation = self.foundation_as_vec();
        let freecells = self.freecells_as_vec();
        let mut tableau: Vec<Vec<Card>> = vec![];

        // Needed in various places.
        let empty_card = Card::new(0, Suit::None);

        // This determines the max number of rows we'll need to work on displaying.
        let max_height = self.columns.iter().map(|col| col.len()).max().unwrap_or(0);
        // Row by row we can create a series of card elements and push them onto a vec for display

        // First insert an initial row of "column label" cards.
        let mut header_row: Vec<Card> = vec![];
        for v in 1..9 {
            let this_card = Card::new(v, Suit::Header);
            header_row.push(this_card);
        }
        tableau.push(header_row);

        for row in 0..max_height {
            let mut this_row: Vec<Card> = vec![];
            for col in &self.columns {
                if row < col.len() {
                    this_row.push(col[row]);
                } else {
                    this_row.push(empty_card);
                }
            }
            
            // Now push this row into the bigger vec
            tableau.push(this_row);
        }

        element! {
            View() {
                View(
                    border_style: BorderStyle::None,
                    // border_color: Color::Green,
                    flex_direction: FlexDirection::Row,
                    padding: 0,
                    margin: 0,
                ) {
                    // Tableau
                    View (
                        flex_direction:FlexDirection::Column, 
                        border_style: BorderStyle::None, border_color: Color::Red,
                        margin_right: 3, margin_top: 1,
                    ){
                        #(  
                            tableau.iter().map(|row| {
                                element!{
                                    View(border_style: BorderStyle::None, border_color: Color::Blue){
                                        #(
                                            row.iter().map(|this_card| {
                                                this_card.as_element()
                                            })
                                        )
                                    }
                                }
                            })
                        )
                    }                    

                    // Right side panels
                    View(
                        // border_style: BorderStyle::Round,
                        // border_color: Color::Red,
                        flex_direction: FlexDirection::Column,
                    ) {
                        View(border_style: BorderStyle::Single, border_color: Color::Green){
                            View(width:12){ Text(content:"Foundations")}
                            #(
                                foundation.iter().map(|this_card| {
                                    this_card.as_element()
                                })
                            )
                        }                        
                        View(border_style: BorderStyle::Single, border_color: Color::Blue){
                            View(width:12){ Text(content:"Freecells")}
                            #(
                                freecells.iter().map(|this_card| {
                                    this_card.as_element()
                                })
                            )
                        }
                        View(
                            border_style: last_move_border,
                            border_color: last_move_color,
                        ) {
                            Text(content: format!("{}",self.str_last_move()))
                        }

                    }
                } 
            }
            
        }.print();

        Ok(())
    }

}


impl GameState {
    pub fn str_last_move(&self) -> String {
        let mut retval = String::new();

        match &self.last_move_error {
            Some(e) => {
                retval.push_str(&e.to_string());
            },
            None => {
                // Get last move off the history stack
                let last_move = self.get_last_move();
                if let Some(m) = last_move {
                    retval.push_str(&format!("Last Move: {}", m));
                }
            }
        };

        retval
    }

    pub fn freecells_as_vec(&self) -> Vec<Card> {
        let empty_card = Card::new(0, Suit::None);
        let mut vec_fnds: Vec<Card> = vec![];
        for cell in &self.freecells {
            match cell {
                Some(card) => vec_fnds.push(*card),
                None => vec_fnds.push(empty_card),
            }
        }
        vec_fnds
    }

    pub fn foundation_as_vec(&self) -> Vec<Card> {
        let empty_card = Card::new(0, Suit::None);
        let mut vec_fnds: Vec<Card> = vec![];
        for cell in &self.foundations {
            match cell {
                Some(card) => vec_fnds.push(*card),
                None => vec_fnds.push(empty_card),
            }
        }
        vec_fnds
    }

    /// Resets gamestate to an empty board with cards dealt into the columns.
    pub fn reset(seed: Option<u64>) -> GameState {
        let mut columns: [Vec<Card>; 8] = Default::default();

        // If the seed is none, we need to randomly generate our own so we can record it
        let seed = seed.unwrap_or_else(|| OsRng.next_u64());

        let deck = Self::generate_shuffled_deck(seed); 
        // Walk each of the cards, they come preshuffled, and stuff them into the columns.
        for (i, card) in deck.into_iter().enumerate() {
            columns[i % 8].push(card);
        }

        GameState {
            freecells: [None, None, None, None],
            foundations: [None, None, None, None],
            columns,
            history: Vec::new(),
            seed,
            last_move_error: None,
        }

    }

    /// Generates a shuffled deck with the rng fed via the supplied seed argument.
    pub fn generate_shuffled_deck(seed: u64) -> Vec<Card> {
        let mut deck = Vec::with_capacity(52);

        // The ordered deck.
        for &suit in &[Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds] {
            for rank in 1..=13 {
                deck.push(Card{ rank, suit});
            }
        }
        
        // Shuffle according to the seed.
        let mut rng = StdRng::seed_from_u64(seed);

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

    /// Does the real checking of a move to see if it is valid
    /// 
    /// - Can only move from a non-empty location
    /// - Can only move onto a stack where the color is different and the rank is higher
    ///   (ie: red 2 can stack onto a black 3)
    /// - Foundations must go up in order
    /// - Freecells must be empty
    pub fn check_move(&mut self, mv: &Move) -> Result<(), String> {
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

                let card: &Card = match mv.from {
                    LocationType::Column => {
                        let src = self.columns.get(mv.from_idx).ok_or("Invalid source column.")?;
                        src.last().ok_or("Source column is empty.")?
                    },
                    LocationType::Freecell => {
                        &self.freecells.get_mut(mv.from_idx)
                            .ok_or("Invalid freecell index.")?
                            .ok_or("Freecell is empty..")?
                    },
                    _ => Err("Invalid source location for a move to foundation")?
                };

                let index = match card.suit {
                    Suit::Spades => 0,
                    Suit::Hearts => 1,
                    Suit::Diamonds => 2,
                    Suit::Clubs => 3,
                    _ => Err("Invalid source location for a move to foundation")?
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

    /// Simply records a provided move into the game state's history vec.
    pub fn record_move(&mut self, mv: &Move) -> Result<(), String> {
        self.history.push(mv.clone());

        Ok(())
    }

    /// Does the logical application of a move, but only after it checks it for validity first.
    pub fn apply_move(&mut self, mut mv: Move) -> Result<(), String> {
        self.check_move(&mv)?; // If a move is not valid, we'll bubble up the error with a reason.

        match (mv.from, mv.to) {
            (LocationType::Column, LocationType::Column) => {
                // Col to Col move.
                let card = self.pop_card_from_column(mv.from_idx)?;

                self.columns.get_mut(mv.to_idx)
                    .ok_or("Target column invalid.")?
                    .push(card);

            },
            (LocationType::Column, LocationType::Freecell) => {
                let card =  self.pop_card_from_column(mv.from_idx)?;

                let cell = self.freecells.get_mut(mv.to_idx)
                    .ok_or("Invalid target freecell.")?;

                if cell.is_some() {
                    return Err("Freecell is already occupied.".into());
                }

                *cell = Some(card);
            },
            (LocationType::Column, LocationType::Foundation) => {
                let card = self.pop_card_from_column(mv.from_idx)?;

                // Update the to index based on suit
                let index = match card.suit {
                    Suit::Spades => 0,
                    Suit::Hearts => 1,
                    Suit::Diamonds => 2,
                    Suit::Clubs => 3,
                    _ => Err("Invalid source location for a move to foundation")?
                };

                // Update the move with the updated index
                mv.to_idx = index;

                self.place_in_foundation(card)?
            },
            (LocationType::Freecell, LocationType::Column) => {
                let card = self.take_card_from_freecell(mv.from_idx)?;

                self.columns.get_mut(mv.to_idx)
                    .ok_or("Invalid destination column")?
                    .push(card);
            },
            (LocationType::Freecell, LocationType::Foundation) => {
                let card = self.take_card_from_freecell(mv.from_idx)?;

                // Update the to index based on suit
                let index = get_foundation_index(card);

                // Update the move with the updated index
                mv.to_idx = index;

                self.place_in_foundation(card)?
            },

            _ => return Err("Unsupported move combination".into()),
        }

        // If we got past our check, let's record the move in the history
        self.record_move(&mv)?;

        Ok(())

    }

    /// Pops (pop()) a card from the column at index provided.
    pub fn pop_card_from_column(&mut self, idx: usize) -> Result<Card, String> {
         Ok(self.columns.get_mut(idx)
            .ok_or("Invalid source column")?.pop()
            .ok_or("Source column is empty.")?)
    }

    /// Takes (take()) a card from the freecell at index provided.
    pub fn take_card_from_freecell(&mut self, idx: usize) -> Result<Card, String> {
        Ok(self.freecells.get_mut(idx)
            .ok_or("Invalid freecell index.")?
            .take()
            .ok_or("Freecell is empty.")?)
    }

    /// Places a supplied Card into the foundation
    fn place_in_foundation(&mut self, card: Card) -> Result<(), String> {
        let index = get_foundation_index(card);

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

    /// Copies the last move off of the stack for review on the CLI when printing. 
    /// Just a convenience method, really.
    pub fn get_last_move(&self) -> Option<&Move> {
        self.history.last()
    }

    pub fn undo(&mut self) -> Result<(), String> {
        if let Some(m) = self.history.pop() {
            // Revert the last move before we apply it.
            let u = Move {
                from: m.to,
                from_idx: m.to_idx,
                to: m.from,
                to_idx: m.from_idx
            };
            println!("Undoing move {} ({})", m, u);
            self.force_move(u)?;
        } else {
            println!("No moves to undo.");
        }
        Ok(())
    }

    // Applies a move in a more "forced" manner, used when reverting.  See undo().
    pub fn force_move(&mut self, u: Move) -> Result<(), String> {
        let card = match u.from {
            LocationType::Column => {self.columns.get_mut(u.from_idx)
                .ok_or("Invalid source column")?.pop()
                .ok_or("Source column is empty.")?
            },
            LocationType::Freecell => {self.freecells.get_mut(u.from_idx)
                .ok_or("Invalid freecell index.")?
                .take()
                .ok_or("Freecell is empty.")?
            },
            LocationType::Foundation => {self.foundations.get_mut(u.from_idx)
                .ok_or("Invalid foundation index.")?
                .take()
                .ok_or(format!("Foundation is empty at index {}.", u.from_idx))?
            },
        };

        match u.to {
            LocationType::Column => {
                self.columns.get_mut(u.to_idx)
                    .ok_or("Invalid destination column")?
                    .push(card);
                
                Ok(())
            },
            LocationType::Freecell => {
                let cell = self.freecells.get_mut(u.to_idx)
                    .ok_or("Invalid target freecell.")?;

                if cell.is_some() {
                    return Err("Freecell is already occupied.".into());
                }

                *cell = Some(card);

                Ok(())
            },
            LocationType::Foundation => {
                self.place_in_foundation(card)?;
                Ok(())
            }
        }

    }
}
