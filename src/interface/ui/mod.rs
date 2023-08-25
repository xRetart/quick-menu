pub mod colorscheme;
pub mod options_list;

use {
    crate::parse::MenuOption,
    tui::{backend::Backend, layout::Rect, Frame},
};
pub use {colorscheme::Colorscheme, options_list::OptionsList};

pub struct Ui<'o> {
    pub options: OptionsList<'o>,
    last_area: Option<Rect>,
}
impl<'o> Ui<'o> {
    pub fn new(options: Vec<MenuOption>, colorscheme: &Colorscheme) -> Self {
        let options = OptionsList::new(options, colorscheme);
        let last_area = None;
        Self { options, last_area }
    }
    fn option_at(&self, row: u16, column: u16) -> Option<usize> {
        let in_range = |x, (min, offset)| (min..=min + offset).contains(&x);
        self.last_area.and_then(|area| {
            (in_range(column, (area.x, area.width)) && in_range(column, (area.x, area.width)))
                .then_some((row - area.y - 1) as usize)
        })
    }
    pub fn select(&mut self, row: u16, column: u16) -> Option<usize> {
        let position = self.option_at(row, column);
        if position == self.options.state.selected() {
            position
        } else {
            self.options.state.state.select(position);
            None
        }
    }
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let size = frame.size();
        let centered = self.center_options(size);
        let options = self.options.list.clone();
        let state = &mut self.options.state.state;

        self.last_area = Some(centered);
        frame.render_stateful_widget(options, centered, state);
    }
    fn center_options(&self, outer: Rect) -> Rect {
        let width = self.options.width + 2;
        let height = self.options.height + 2;

        let x = (outer.width - width) / 2;
        let y = (outer.height - height) / 2;

        Rect {
            x,
            y,
            width,
            height,
        }
    }
}
