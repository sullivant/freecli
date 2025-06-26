use clap::Parser;
// use freecli::card::{Card, Suit};
use freecli::gamestate::GameState;
use freecli::io::{load_game, save_game, delete_game};
use freecli::moves::{LocationType, Move};
use freecli::cli::{AppArgs};
use std::io::{self, Write};
use std::process;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_file = "game_state.json";
    let args = AppArgs::parse();

    println!("{:?}", args);

    if args.reset {
        println!("Resetting save game file and starting fresh.");
        let _ = delete_game(game_file);
    }

    let mut game = match load_game(game_file) {
        Ok(g) => g,
        Err(_) => {
            println!("No save game found.  Creating new.");
            GameState::reset()
        }
    };
    
    let mv = match Move::from_args(&args.ft, args.fi, &args.tt, args.ti) {
        Ok(mv) => mv,
        Err(e) => {
            eprintln!("Invalid Move: {}", e);
            process::exit(1);
        }
    };

    // If we are not just printing, we should apply this move, too.
    if !args.print {
        println!("Applying move.\n\n");
    }
    
    println!("Current game:");
    println!("{}", game);

    cleanup(game, game_file)?;

    Ok(())
}


pub fn cleanup(game: GameState, game_file: &str) -> Result<(), Box<dyn std::error::Error>>  {
    println!("Saving game state to file {} and exiting...", game_file);
    save_game(&game, game_file)
}