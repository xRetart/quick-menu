pub mod colors;
pub mod input_mode;
pub mod vector;
pub mod widgets;

pub use colors::Colorscheme;
pub use input_mode::InputMode;
use ratatui::{backend::Backend, prelude::Rect, Frame};
pub use vector::Vector;
pub use widgets::list::List;

use self::widgets::{list::Customizations, searchbar::Searchbar};
use crate::parse::MenuOption;
pub struct Ui<'o> {
    pub list: List<'o>,
    pub searchbar: Searchbar,
    pub input_mode: InputMode,
}
impl<'o> Ui<'o> {
    pub fn new(options: &'o [MenuOption], customizations: Customizations) -> Self {
        let list = List::new(options, customizations);
        let query = Searchbar::new();
        let input_mode = InputMode::Selecting;

        Self { list, searchbar: query, input_mode }
    }
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let area = frame.size();
        let query_height = 3;
        if area.height <= query_height {
            self.input_mode = InputMode::Selecting;
        }

        match self.input_mode {
            InputMode::Searching => {
                let list_bounds = Vector { x: area.width, y: area.height - query_height };
                let list_area = self.list.dimensions.center_in(list_bounds);
                let query_area = Rect {
                    x: list_area.x,
                    y: list_area.y + list_area.height,
                    height: query_height,
                    width: list_area.width,
                };

                self.list.render(frame, list_area, Some(&self.searchbar.query));
                self.searchbar.render(frame, query_area);
            },
            InputMode::Selecting => {
                let list_bounds = Vector { x: area.width, y: area.height };
                let list_area = self.list.dimensions.center_in(list_bounds);
                self.list.render(frame, list_area, None);
            },
        }
    }
    pub fn append_query(&mut self, character: char) {
        self.searchbar.query.push(character);
        self.update_query();
    }

    pub fn pop_query(&mut self) {
        self.searchbar.query.pop();
        self.update_query();
    }
    fn update_query(&mut self) {
        self.list.query(&self.searchbar.query);
    }
}
