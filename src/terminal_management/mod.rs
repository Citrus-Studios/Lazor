use std::io::stdout;

use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
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
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    }

    // Runs the event loop for the terminal
    pub fn run(&mut self, function: fn() -> anyhow::Result<()>) {
        loop {
            function().unwrap();
        }
    }
}

// Destroys the struct and reverts terminal
impl Drop for TerminalManager {
    fn drop(&mut self) {
        disable_raw_mode();
    }
}
