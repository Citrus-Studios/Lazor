/// This struct is used for managing the terminal
pub struct TerminalManager {}

impl TerminalManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self, function: fn() -> anyhow::Result<()>) {
        loop {
            function().unwrap();
        }
    }
}
