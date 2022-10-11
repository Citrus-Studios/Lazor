pub type Callbacks = Vec<Callback>;

pub enum Callback {
    Print(String),
    Goto(u32, u32),
}
