use ratatui::{
    prelude::{Backend, Rect},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::interface::ui::Customizations;

pub struct Searchbar {
    pub query: String,
    customizations: Customizations,
}

impl Searchbar {
    pub const fn new(customizations: Customizations) -> Self {
        let string = String::new();
        Self { query: string, customizations }
    }
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, destination: Rect) {
        let block = self.customizations.borders(Block::new());
        let scroll = self.scroll(destination.width);
        let widget = Paragraph::new(self.query.as_str()).block(block).scroll((0, scroll));
        frame.render_widget(widget, destination);

        let x = destination.x + self.cursor(destination.width);
        let y = destination.y + 1;
        frame.set_cursor(x, y);
    }
    fn cursor(&self, width: u16) -> u16 {
        let length = u16::try_from(self.query.len()).unwrap();
        (length + 1).min(width - 3)
    }
    fn scroll(&self, width: u16) -> u16 {
        let length = i64::try_from(self.query.len()).unwrap();
        let scroll = (length - i64::from(width) + 4).max(0);
        u16::try_from(scroll).unwrap()
    }
}
