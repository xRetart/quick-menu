pub mod colorscheme;
pub mod list;

pub use colorscheme::Colorscheme;
pub use list::List;
use ratatui::{backend::Backend, Frame};

use self::list::Customizations;
use crate::parse::MenuOption;

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}
pub struct Ui<'o> {
    pub list: List<'o>,
}
impl<'o> Ui<'o> {
    pub fn new(options: &'o [MenuOption], customizations: Customizations) -> Self {
        let list = List::new(options, customizations);

        Self { list }
    }
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        self.list.render(frame);
    }
}
