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
            (KeyCode::Char('w'), _) => Ok(vec![Callback::MoveBy(0, -1)]),
            (KeyCode::Char('s'), _) => Ok(vec![Callback::MoveBy(0, 1)]),
            (KeyCode::Char('a'), _) => Ok(vec![Callback::MoveBy(-1, 0)]),
            (KeyCode::Char('d'), _) => Ok(vec![Callback::MoveBy(1, 0)]),
            _ => Ok(vec![]),
        },
        _ => Ok(vec![]),
    });
}
