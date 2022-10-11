use std::{
    cmp::{max, min},
    io::stdout,
    process::exit,
};

use crossterm::{
    cursor,
    event::{read, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use self::callbacks::Callbacks;

pub mod callbacks;

/// This struct is used for managing the terminal
pub struct TerminalManager {
    x: u16,
    y: u16,
}

impl TerminalManager {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    // Prepares the terminal
    fn prepare(&mut self) {
        // enter raw mode and clear the screen and moves cursor to 0, 0
        enable_raw_mode();
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    }

    fn tasks(&mut self) {
        execute!(stdout(), cursor::MoveTo(self.x, self.y)).unwrap();
    }

    // Runs the event loop for the terminal
    pub fn run(&mut self, function: fn(Event) -> anyhow::Result<Callbacks>) {
        self.prepare();
        loop {
            for x in function(read().unwrap()).unwrap() {
                match x {
                    callbacks::Callback::Print(msg) => {
                        print!("{}", msg);
                    }
                    callbacks::Callback::Quit(x) => exit(x),
                    callbacks::Callback::MoveTo(x, y) => {
                        self.x = x;
                        self.y = y;
                    }
                    callbacks::Callback::MoveBy(x, y) => {
                        let tmp_x = self.x as i32 + x;
                        let tmp_y = self.y as i32 + y;

                        self.x = min(max(tmp_x, 0), u16::MAX as i32) as u16;
                        self.y = min(max(tmp_y, 0), u16::MAX as i32) as u16;
                    }
                }
                self.tasks();
            }
        }
    }
}

// Destroys the struct and reverts terminal
impl Drop for TerminalManager {
    fn drop(&mut self) {
        disable_raw_mode();
    }
}
