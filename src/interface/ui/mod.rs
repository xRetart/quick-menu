pub mod colorscheme;
pub mod list;
pub mod query;

pub use colorscheme::Colorscheme;
pub use list::List;
use ratatui::{
    backend::Backend,
    prelude::{Constraint, Layout},
    Frame,
};

use self::{list::Customizations, query::Query};
use crate::parse::MenuOption;

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

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}
pub struct Ui<'o> {
    pub list: List<'o>,
    pub query: Query,
    pub input_mode: InputMode,
}
impl<'o> Ui<'o> {
    pub fn new(options: &'o [MenuOption], customizations: Customizations) -> Self {
        let list = List::new(options, customizations);
        let query = Query::new();
        let input_mode = InputMode::Selecting;

        Self { list, query, input_mode }
    }
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        match self.input_mode {
            InputMode::Searching => {
                let layout =
                    Layout::default().constraints([Constraint::Min(1), Constraint::Max(3)]);
                let chunks = layout.split(frame.size());

                self.list.render(frame, chunks[0]);
                self.query.render(frame, chunks[1]);
            },
            InputMode::Selecting => self.list.render(frame, frame.size()),
        }
    }
    pub fn append_query(&mut self, character: char) {
        self.query.string.push(character);
        self.list.query(&self.query.string);
    }
}
