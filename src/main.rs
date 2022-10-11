mod terminal_management;

use anyhow::Ok;
use clap::Parser;
use crossterm::event::{Event, KeyCode};
use terminal_management::{callbacks::Callback, TerminalManager};

/// Args to the program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub file: String,
}

fn main() {
    let args = Args::parse();

    TerminalManager::new().run(|event| match event {
        Event::Key(k) => match (k.code, k.modifiers) {
            (KeyCode::Char('q'), _) => Ok(vec![Callback::Quit(0)]),
            _ => Ok(vec![]),
        },
        _ => Ok(vec![]),
    });
}
