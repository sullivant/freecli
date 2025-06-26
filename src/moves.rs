use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    pub fn from_args( from: &str, from_idx: usize, to: &str, to_idx: usize ) -> Result<Self, String> {
        let from = parse_location_type(from)?;
        let to = parse_location_type(to)?;

        let to_idx = match to {
          LocationType::Foundation => 0, // or from suit later
          _ => to_idx,
        };

        Ok(Self {
            from: from,
            from_idx: from_idx,
            to: to,
            to_idx: to_idx,
        })
    }
}


pub fn parse_location_type(input: &str) -> Result<LocationType, String> {
    match input.to_lowercase().as_str() {
        "col" | "column" => Ok(LocationType::Column),
        "cell" | "freecell" => Ok(LocationType::Freecell),
        "fnd" | "foundation"  => Ok(LocationType::Freecell),
        _ => Err(format!("Invalid location type: {}", input)),
    }
}

