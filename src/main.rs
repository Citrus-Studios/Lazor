mod terminal_mangament;

use clap::Parser;
use terminal_mangament::TerminalManager;

/// Args to the program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub file: String,
}

fn main() {
    let args = Args::parse();

    TerminalManager::new().run(|| Ok(()));
}
