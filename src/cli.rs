
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "freecli")]
#[command(about = "A Freecell CLI interface.", long_about = None)]
pub struct AppArgs {
    #[arg(long)]
    pub reset: bool,

    #[arg(long)]
    pub print: bool,

    #[arg(long)]
    pub stats: bool,

    #[arg(long)]
    pub history: bool,

    #[arg(long)]
    pub undo: bool,

    #[arg(long)]
    pub seed: Option<u64>,

    #[arg(value_enum)]
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