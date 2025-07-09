use std::fs::{self, remove_file, File, OpenOptions};
use std::io::{Read, Write, ErrorKind, self};
use std::env;
use std::path::PathBuf;
use crate::gamestate::GameState;
use crate::stats::GameStats;
use serde_json;

static GAME_STATS_FILE: &str = ".freecli_stats.json";
static GAME_STATE_FILE: &str = ".freecli_state.json";

/// Returns the appropriate path to use when considering saves, loads, etc.
/// 
/// Will default to a user's home directory and if unable to will use the
/// directory from which the application was executed.
pub fn target_path() -> String {
    let home_dir = dirs::home_dir();
    let target_dir: PathBuf;

    if let Some(home) = home_dir {
        if fs::metadata(&home).map(|m| m.permissions().readonly()).unwrap_or(true) {
            target_dir = env::current_dir().expect("Failed to get current directory");
        } else {
            target_dir = home;
        }
    } else {
        target_dir = env::current_dir().expect("Failed to get current directory");
    }

    match target_dir.as_os_str().to_str() {
        Some(s) => s.to_string(),
        _ => "".to_string()
    }
}

pub fn load_stats() -> io::Result<GameStats> {
    let target = target_path();
    println!("Using target:{}",target);


    // Loads .game_stats.json
    match fs::read_to_string(GAME_STATS_FILE) {
        Ok(content) => Ok(serde_json::from_str(&content)?), // Load if ok to load...
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(GameStats::default()), // Create new if not found.
        Err(e) => Err(e), // Bubble up any other error.  Permissions, etc.
    }
}

pub fn save_stats(stats: &GameStats) -> io::Result<()> {
    let json = serde_json::to_string_pretty(stats)?;
    fs::write(GAME_STATS_FILE, json)
}

pub fn save_game(game: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(game)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(GAME_STATE_FILE)?;
    
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_game() -> Result<GameState, Box<dyn std::error::Error>> {
    let mut file = File::open(GAME_STATE_FILE)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let game: GameState = serde_json::from_str(&contents)?;
    Ok(game)
}

pub fn delete_game() -> Result<(), Box<dyn std::error::Error>> {
    remove_file(GAME_STATE_FILE)?;
    Ok(())
}