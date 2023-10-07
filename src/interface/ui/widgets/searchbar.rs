use ratatui::{
    prelude::{Backend, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub struct Searchbar {
    pub query: String,
}

impl Searchbar {
    pub const fn new() -> Self {
        let string = String::new();
        Self { query: string }
    }
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, destination: Rect) {
        let block = Block::new().borders(Borders::ALL).border_type(BorderType::Thick);
        let scroll = self.scroll(destination.width);
        let widget = Paragraph::new(self.query.clone()).block(block).scroll((0, scroll));
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
