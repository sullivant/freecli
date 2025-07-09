use std::fs::{self, remove_file, File, OpenOptions};
use std::io::{Read, Write, ErrorKind, self};
use std::env;
use std::path::{MAIN_SEPARATOR, PathBuf};
use crate::gamestate::GameState;
use crate::stats::GameStats;
use serde_json;

static GAME_ROOT: &str = ".freecli";
static GAME_STATS_FILE: &str = "freecli_stats.json";
static GAME_STATE_FILE: &str = "freecli_state.json";


/// Returns the appropriate path to use when considering saves, loads, etc.
/// 
/// Will default to a user's home directory and if unable to will use the
/// directory from which the application was executed.
pub fn target_path(path: &str) -> String {
    let home_dir = dirs::home_dir();
    let mut target_dir: PathBuf;

    if let Some(home) = home_dir {
        if fs::metadata(&home).map(|m| m.permissions().readonly()).unwrap_or(true) {
            target_dir = env::current_dir().expect("Failed to get current directory");
        } else {
            target_dir = home;
        }
    } else {
        target_dir = env::current_dir().expect("Failed to get current directory");
    }

    // Push on our game's core root dir
    target_dir.push(GAME_ROOT);

    if !target_dir.exists() {
        match fs::create_dir_all(&target_dir) {
            Ok(_) => println!("Directory created: {}", target_dir.display()),
            Err(e) => eprintln!("Failed to create directory: {}", e),
        }
    } 

    match target_dir.as_os_str().to_str() {
        Some(s) => format!("{}{}{}", s.to_string(), MAIN_SEPARATOR, path),
        _ => "".to_string()
    }
}

pub fn load_stats() -> io::Result<GameStats> {
    let target = target_path(GAME_STATS_FILE);

    match fs::read_to_string(target) {
        Ok(content) => Ok(serde_json::from_str(&content)?), // Load if ok to load...
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(GameStats::default()), // Create new if not found.
        Err(e) => Err(e), // Bubble up any other error.  Permissions, etc.
    }
}

pub fn save_stats(stats: &GameStats) -> io::Result<()> {
    let json = serde_json::to_string_pretty(stats)?;
    let target = target_path(GAME_STATS_FILE);

    fs::write(target, json)
}

pub fn save_game(game: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(game)?;

    let target = target_path(GAME_STATE_FILE);
    
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(target)?;
    
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_game() -> Result<GameState, Box<dyn std::error::Error>> {
    let target = target_path(GAME_STATE_FILE);
    let mut file = File::open(target)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let game: GameState = serde_json::from_str(&contents)?;
    Ok(game)
}

pub fn delete_game() -> Result<(), Box<dyn std::error::Error>> {
    let target = target_path(GAME_STATE_FILE);
    remove_file(target)?;
    Ok(())
}