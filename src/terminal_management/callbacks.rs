pub type Callbacks = Vec<Callback>;

pub enum Callback {
    Print(String),
    Goto(u16, u16),
    Quit(i32),
}
