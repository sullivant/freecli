use serde::{Deserialize, Serialize};

use crate::cli::LocationArg;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LocationType {
    Column,
    Freecell,
    Foundation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub from: LocationType,
    pub from_idx: usize,
    pub to: LocationType,
    pub to_idx: usize,
}

impl Move {
    pub fn from_args( locations: &[LocationArg] ) -> Result<Option<Self>, String> {
        match locations {
            [] => Ok(None), // No move
            [from, to] => {
                let (from_type, from_idx) = convert_location_arg(from);
                let (to_type, to_idx) = convert_location_arg(to);

                Ok(Some(Move {
                    from: from_type, from_idx,
                    to: to_type, to_idx,
                }))
            },
            _ => Err("You must specify two move locations only.".to_string()),
        }
    }
}

pub fn convert_location_arg(arg: &LocationArg) -> (LocationType, usize) {
    use LocationArg::*;
    match arg {
        C0 => (LocationType::Column, 0),
        C1 => (LocationType::Column, 1),
        C2 => (LocationType::Column, 2),
        C3 => (LocationType::Column, 3),
        C4 => (LocationType::Column, 4),
        C5 => (LocationType::Column, 5),
        C6 => (LocationType::Column, 6),
        C7 => (LocationType::Column, 7),

        F0 => (LocationType::Freecell, 0),
        F1 => (LocationType::Freecell, 1),
        F2 => (LocationType::Freecell, 2),
        F3 => (LocationType::Freecell, 3),

        Foundation => (LocationType::Foundation, 0),
    }
}


