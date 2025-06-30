
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "freecli")]
#[command(about = "A Freecell CLI interface.", long_about = None)]
pub struct AppArgs {
    #[arg(long, help = "Reset the game and generate a new board.")]
    pub reset: bool,

    #[arg(long, help = "Simply print the current game state.")]
    pub print: bool,

    #[arg(long, help = "Print more detailed statistics about game wins, attempts, etc.")]
    pub stats: bool,

    #[arg(long, help = "Print a history of moves made for this current game.")]
    pub history: bool,

    #[arg(long, help = "Undo the last move in the game history.")]
    pub undo: bool,

    #[arg(long, help = "Optional, if passed will seed the RNG with the value passed for repeatability.")]
    pub seed: Option<u64>,

    #[arg(value_enum, help = "Possible positions in the format of <from> <to>, ie: \"c0 c3\" or \"c6 foundation\"")]
    pub positions: Vec<LocationArg>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum LocationArg {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    
    F0,
    F1,
    F2,
    F3,

    Foundation,
}