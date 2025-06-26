
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "freecli")]
#[command(about = "A Freecell CLI interface.", long_about = None)]
pub struct AppArgs {
    #[arg(long)]
    pub reset: bool,

    #[arg(long)]
    pub print: bool,

    #[arg(long)]
    pub ft: String,  // "col" or "cell"

    #[arg(long, value_name = "NUM")]
    pub fi: usize,    // zero based
    
    #[arg(long)]
    pub tt: String,    // "col", "cell", or "foundation"

    #[arg(long, value_name = "NUM")]
    pub ti: usize,      // zero based
}
