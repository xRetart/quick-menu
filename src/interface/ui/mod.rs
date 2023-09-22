pub mod colorscheme;
pub mod list;

use {
    crate::parse::MenuOption,
    tui::{backend::Backend, layout::Rect, Frame},
};
pub use {colorscheme::Colorscheme, list::List};

pub struct Ui<'o> {
    pub options: List<'o>,
    area: Option<Rect>,
}
pub struct Customizations {
    pub colorscheme: Colorscheme,
    pub noborders: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}
impl<'o> Ui<'o> {
    pub fn new(options: &'o [MenuOption], customizations: Customizations) -> Self {
        let Customizations {
            colorscheme,
            noborders,
        } = customizations;
        let options_customizations = list::Customizations {
            colorscheme,
            noborders,
        };
        let options = List::new(options, &options_customizations);

        let area = None;

        Self { options, area }
    }
    pub fn select(&mut self, coordinate: Coordinate) -> Option<usize> {
        let position = self.area.and_then(|area| row_in_area(area, coordinate));
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

        self.area = Some(centered);
        frame.render_stateful_widget(options, centered, state);
    }
    const fn center_options(&self, outer: Rect) -> Rect {
        let width = self.options.dimensions.x + 2;
        let height = self.options.dimensions.y + 2;

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
fn row_in_area(area: Rect, coordinate: Coordinate) -> Option<usize> {
    let Coordinate { x, y } = coordinate;

    (area.x..=area.x + area.width)
        .contains(&x)
        .then_some((y - area.y - 1) as usize)
}
