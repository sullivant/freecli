use clap::Parser;
use freecli::gamestate::GameState;
use freecli::io::{load_game, save_game, delete_game};
use freecli::moves::{Move};
use freecli::cli::{AppArgs};
use freecli::stats::GameStats;
use std::process;

static STATS_FILE: &str = ".game_stats.json";


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_file = "game_state.json";

    let mut args = AppArgs::parse();

    // Load the stats
    let mut stats = GameStats::load(STATS_FILE).unwrap_or_default();

    // println!("{:?}", args);

    if args.reset {
        println!("Resetting save game file and starting fresh.");
        let _ = delete_game(game_file);
    }

    let mut game = match load_game(game_file) {
        Ok(g) => g,
        Err(_) => {
            println!("No save game found.  Creating new.");
            args.reset = true;
            stats.record_game_start();
            stats.save(STATS_FILE)?;
            GameState::reset()
        }
    };

    // If we have reset or had to create from scratch, no sense in applying a move.  Just display and save.
    if args.reset {
        println!("{}", game);
        cleanup(game, game_file, stats)?;
        process::exit(1);
    }

    
    let mv = match Move::from_args(&args.positions) {
        Ok(mv) => mv,
        Err(e) => {
            eprintln!("Invalid Move: {}", e);
            println!("{}", game);
            process::exit(1);
        }
    };

    // If we are not just printing, we should apply this move, too.
    if !args.print {
        if mv.is_some() {
            match game.apply_move(mv.unwrap()) {
                Ok(_) => {
                    // Move has been applied ok.  Let's record the move.
                    stats.record_move();
                    println!("{}", game);
                    cleanup(game, game_file, stats)?;
                },
                Err(e) => {
                    eprintln!("Move failed: {}\n", e);
                    println!("{}", game);
                    process::exit(1);
                }
            }
        }
    } else {
        println!("{}", game);
    }
    

    Ok(())
}


pub fn cleanup(game: GameState, game_file: &str, stats: GameStats) -> Result<(), Box<dyn std::error::Error>>  {
    // println!("Saving game state to file {} and exiting...", game_file);
    stats.save(STATS_FILE)?;
    save_game(&game, game_file)
    
}