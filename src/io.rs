use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::gamestate::GameState;
use serde_json;


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