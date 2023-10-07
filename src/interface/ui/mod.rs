pub mod colorscheme;
pub mod list;
pub mod query;

pub use colorscheme::Colorscheme;
pub use list::List;
use ratatui::{backend::Backend, prelude::Rect, Frame};

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
        let area = frame.size();
        let query_height = 3;
        if area.height <= query_height {
            self.input_mode = InputMode::Selecting;
        }

        match self.input_mode {
            InputMode::Searching => {
                let list_area =
                    Rect { x: 0, y: 0, height: area.height - query_height, width: area.width };
                let query_area = Rect {
                    x: 0,
                    y: area.height - query_height,
                    height: query_height,
                    width: area.width,
                };

                self.list.render(frame, list_area, Some(&self.query.string));
                self.query.render(frame, query_area);
            },
            InputMode::Selecting => self.list.render(frame, frame.size(), None),
        }
    }
    pub fn append_query(&mut self, character: char) {
        self.query.string.push(character);
        self.update_query();
    }

    pub fn pop_query(&mut self) {
        self.query.string.pop();
        self.update_query();
    }
    fn update_query(&mut self) {
        self.list.query(&self.query.string);
    }
}
