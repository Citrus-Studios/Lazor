pub type Callbacks = Vec<Callback>;

pub enum Callback {
    Print(String),

    MoveTo(u16, u16),
    MoveBy(i32, i32),

    Quit(i32),
}
