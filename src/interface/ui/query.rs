use ratatui::{
    prelude::{Backend, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub struct Query {
    pub string: String,
}

impl Query {
    pub const fn new() -> Self {
        let string = String::new();
        Self { string }
    }
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, destination: Rect) {
        let block = Block::new().borders(Borders::ALL).border_type(BorderType::Thick);
        let widget = Paragraph::new(self.string.clone()).block(block);
        frame.render_widget(widget, destination);

        let x = destination.x + u16::try_from(self.string.len()).unwrap() + 1;
        let y = destination.y + 1;
        frame.set_cursor(x, y);
    }
}
