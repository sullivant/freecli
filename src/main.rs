use clap::Parser;
use freecli::gamestate::GameState;
use freecli::io::{delete_game, load_game, load_stats, save_game, save_stats};
use freecli::moves::{Move};
use freecli::cli::{AppArgs};
use freecli::stats::GameStats;
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut args = AppArgs::parse();

    // Load the stats
    let mut stats = load_stats()?;

    if args.reset {
        println!("Resetting save game file and starting fresh.");
        let _ = delete_game();
    }

    let mut game = match load_game() {
        Ok(g) => g,
        Err(_) => {
            args.reset = true;
            stats.record_game_start();
            save_stats(&stats)?;

            // If we have passed a seed, use that.
            match args.seed {
                Some(_s) => {
                    println!("Creating new with seed: {}", args.seed.unwrap());
                    GameState::reset(args.seed)
                },
                _ => {
                    let g = GameState::reset(None);
                    println!("Created new game with seed: {}", g.seed);
                    g
                },
            }
        }
    };

    // If we are just printing, let's clear the last error because there is none. Then print.
    if args.print {
        game.last_move_error = None;
        cleanup(&game, &stats, &args)?;
        process::exit(1);
    }

    // Just a stats print.
    if args.stats {
        println!("{}", stats);
        cleanup(&game, &stats, &args)?;
        process::exit(1);
    }
    
    // Print move history
    if args.history {
        if game.history.is_empty() {
            println!("No History.");
            process::exit(1);
        }
        println!("History (oldest first): ");
        for mv in game.history.iter() {
            println!("{}", mv);
        }

        process::exit(1);
    }

    // Undo last move
    if args.undo {
        game.undo()?;
        cleanup(&game, &stats, &args)?;
        process::exit(1);
    }

    let mv = match Move::from_args(&args.positions) {
        Ok(mv) => mv,
        Err(e) => {
            game.last_move_error = format!("Invalid Move: {}", e).into();
            // println!("{}", game);
            // process::exit(1);
            None
        }
    };

    // If move is something (is_some()) - let's try to apply it.
    if mv.is_some() {
        match game.apply_move(mv.unwrap()) {
            Ok(_) => {
                // Move has been applied ok.  Let's record the lack "None" of an error.
                game.last_move_error = None;
                stats.record_move();
            },
            Err(e) => {
                // Move failed.  It's not applied.  But let's update status.
                game.last_move_error = format!("Move failed: {}", e).into();
            }
        }
    }
    
    // But if we have won, print that too!  Only check if this move is not an error anyway.
    
    if game.last_move_error.is_none() && game.is_win() {
        if !args.json { println!("\u{1F389} You won!"); }
        stats.record_win();
    }

    // Always going to cleanup and print before we leave.
    cleanup(&game, &stats, &args)?;

    Ok(())
}


pub fn cleanup(game: &GameState, stats: &GameStats, args: &AppArgs) -> Result<(), Box<dyn std::error::Error>>  {
   
    if args.json {
        println!("{}", serde_json::to_string_pretty(&game).unwrap());
    } else { 
        println!("{}", game);
    }

    save_stats(stats)?;
    save_game(game)
    
}