use std::fs::{self, remove_file, File, OpenOptions};
use std::io::{Read, Write, ErrorKind, self};
use crate::gamestate::GameState;
use crate::stats::GameStats;
use serde_json;


pub fn load_stats(path: &str) -> io::Result<GameStats> {
    // Loads .game_stats.json
    match fs::read_to_string(path) {
        Ok(content) => Ok(serde_json::from_str(&content)?), // Load if ok to load...
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(GameStats::default()), // Create new if not found.
        Err(e) => Err(e), // Bubble up any other error.  Permissions, etc.
    }
}

pub fn save_stats(stats: &GameStats, path: &str) -> io::Result<()> {
    let json = serde_json::to_string_pretty(stats)?;
    fs::write(path, json)
}

pub fn save_game(game: &GameState, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(game)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_game(path: &str) -> Result<GameState, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let game: GameState = serde_json::from_str(&contents)?;
    Ok(game)
}

pub fn delete_game(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    remove_file(path)?;
    Ok(())
}