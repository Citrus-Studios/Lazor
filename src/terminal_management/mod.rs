mod terminal_manager;

use std::io::stdout;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, Clear, ClearType},
};

/// This struct is used for managing the terminal
pub struct TerminalManager {}

impl TerminalManager {
    pub fn new() -> Self {
        Self {}
    }

    // Prepares the terminal
    fn prepare(&mut self) {
        // enter raw mode and clear the screen and moves cursor to 0, 0
        enable_raw_mode();
        execute!(stdout(), Clear(ClearType::All), Cursor::MoveTo(0, 0))
    }

    pub fn run(&mut self, function: fn() -> anyhow::Result<()>) {
        loop {
            function().unwrap();
        }
    }
}
