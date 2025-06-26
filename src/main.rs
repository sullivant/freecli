// use freecli::card::{Card, Suit};
use freecli::gamestate::GameState;
use freecli::io::{load_game, save_game};
use freecli::moves::{LocationType, Move};
use std::io::{self, Write};
use std::panic::Location;
use std::process;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(name = "freecli")]
#[command(about = "A Freecell CLI interface.", long_about = None)]
pub struct MoveArgs {
    #[arg(long)]
    pub ft: String,  // "col" or "cell"

    #[arg(long, value_name = "NUM")]
    pub fi: usize,    // zero based
    
    #[arg(long)]
    pub tt: String,    // "col", "cell", or "foundation"

    #[arg(long, value_name = "NUM")]
    pub ti: usize,      // zero based
}


pub fn args_to_move(args: MoveArgs) -> Result<Move, String> {
    let from = parse_location_type(&args.ft)?;
    let to = parse_location_type(&args.tt)?;

    let to_idx = match to {
        LocationType::Foundation => 0, // or from suit later
        _ => args.ti,
    };

    Ok(Move {
        from,
        from_idx: args.fi,
        to,
        to_idx,
    })
}

pub fn parse_location_type(input: &str) -> Result<LocationType, String> {
    match input.to_lowercase().as_str() {
        "col" | "column" => Ok(LocationType::Column),
        "cell" | "freecell" => Ok(LocationType::Freecell),
        "fnd" | "foundation"  => Ok(LocationType::Freecell),
        _ => Err(format!("Invalid location type: {}", input)),
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_file = "game_state.json";
    let args = MoveArgs::parse();

    println!("{:?}", args);

    let mut game = match load_game(game_file) {
        Ok(g) => g,
        Err(_) => {
            println!("No save game found, or save corrupted.  Creating new.");
            GameState::reset()
        }
    };
    
    let mv = match args_to_move(args) {
        Ok(mv) => mv,
        Err(e) => {
            eprintln!("Invalid Move: {}", e);
            process::exit(1);
        }
    };
    
    println!("{}", game);


    println!("Saving game state to file {} and exiting...", game_file);
    save_game(&game, game_file)?;

    Ok(())
}
