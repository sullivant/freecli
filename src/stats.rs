use std::fmt::{self, Display, Formatter};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, ErrorKind};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GameStats {
    pub total_moves: usize,
    pub total_games_started: usize,
    pub total_games_won: usize,
}

impl GameStats {
    // Loads .game_stats.json
    pub fn load(path: &str) -> io::Result<Self> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(serde_json::from_str(&content)?), // Load if ok to load...
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(GameStats::default()), // Create new if not found.
            Err(e) => Err(e), // Bubble up any other error.  Permissions, etc.
        }
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)
    }

    pub fn record_move(&mut self) {
        self.total_moves += 1;
    }

    pub fn record_game_start(&mut self) {
        self.total_games_started += 1;
    }

    pub fn record_win(&mut self) {
        self.total_games_won += 1;
    }

}