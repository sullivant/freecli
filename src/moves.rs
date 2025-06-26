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