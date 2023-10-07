#[derive(Clone, Copy)]
pub enum InputMode {
    Selecting,
    Searching,
}
impl InputMode {
    pub fn switch(&mut self) {
        *self = match *self {
            Self::Searching => Self::Selecting,
            Self::Selecting => Self::Searching,
        };
    }
}
