
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "freecli")]
#[command(about = "A Freecell CLI interface.", long_about = None)]
pub struct AppArgs {
    #[arg(long)]
    pub reset: bool,

    #[arg(long)]
    pub print: bool,

    // Columns
    #[arg(long)]
    pub c0: bool,
    #[arg(long)]
    pub c1: bool,
    #[arg(long)]
    pub c2: bool,
    #[arg(long)]
    pub c3: bool,
    #[arg(long)]
    pub c4: bool,
    #[arg(long)]
    pub c5: bool,
    #[arg(long)]
    pub c6: bool,
    #[arg(long)]
    pub c7: bool,
    
    // Free cells
    #[arg(long)]
    pub f0: bool,
    #[arg(long)]
    pub f1: bool,
    #[arg(long)]
    pub f2: bool,
    #[arg(long)]
    pub f3: bool,

    // Foundations
    #[arg(long)]
    pub foundation: bool,

}
