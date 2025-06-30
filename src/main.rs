use clap::Parser;
use freecli::gamestate::GameState;
use freecli::io::{load_game, save_game, delete_game};
use freecli::moves::{Move};
use freecli::cli::{AppArgs};
use std::process;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_file = "game_state.json";
    let mut args = AppArgs::parse();

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
            GameState::reset()
        }
    };

    // If we have reset or had to create from scratch, no sense in applying a move.  Just display and save.
    if args.reset {
        println!("{}", game);
        cleanup(game, game_file)?;
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
                    println!("{}", game);
                    cleanup(game, game_file)?;
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


pub fn cleanup(game: GameState, game_file: &str) -> Result<(), Box<dyn std::error::Error>>  {
    // println!("Saving game state to file {} and exiting...", game_file);
    save_game(&game, game_file)
}