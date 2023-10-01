pub mod colorscheme;
pub mod list;

pub use colorscheme::Colorscheme;
use list::BorderStyle;
pub use list::List;
use tui::{backend::Backend, layout::Rect, Frame};

use crate::parse::MenuOption;

#[derive(Clone, Copy)]
pub struct Customizations {
    pub colorscheme: Colorscheme,
    pub border_style: BorderStyle,
}

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}
pub struct Ui<'o> {
    pub options: List<'o>,
    area: Option<Rect>,
}
impl<'o> Ui<'o> {
    pub fn new(options: &'o [MenuOption], customizations: Customizations) -> Self {
        let Customizations { colorscheme, border_style } = customizations;
        let options_customizations = list::Customizations { colorscheme, border_style };
        let options = List::new(options, options_customizations);

        let area = None;

        Self { options, area }
    }
    pub fn select(&mut self, coordinate: Coordinate) -> Option<usize> {
        let position = self.area.and_then(|area| row_in_area(area, coordinate));
        if position == self.options.state.selected() {
            position
        }
        else {
            self.options.state.state.select(position);
            None
        }
    }
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let size = frame.size();
        self.options.fit(size.width);
        let centered = self.center_options(size);
        let options = self.options.list.clone();
        let state = &mut self.options.state.state;

        self.area = Some(centered);
        frame.render_stateful_widget(options, centered, state);
    }
    fn center_options(&self, outer: Rect) -> Rect {
        let width = self.options.dimensions.x.min(outer.width);
        let height = self.options.dimensions.y.min(outer.height);

        let x = (outer.width - width) / 2;
        let y = (outer.height - height) / 2;

        Rect { x, y, width, height }
    }
}
fn row_in_area(area: Rect, coordinate: Coordinate) -> Option<usize> {
    let Coordinate { x, y } = coordinate;

    (area.x ..= area.x + area.width).contains(&x).then_some((y - area.y - 1) as usize)
}
