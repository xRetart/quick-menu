pub use ratatui::style::Color;

use crate::args::Cli;

#[derive(Clone, Copy)]
pub struct CellColor {
    pub foreground: Color,
    pub background: Color,
}

#[derive(Clone, Copy)]
pub struct Colorscheme {
    pub selected: CellColor,
    pub key: CellColor,
    pub border: Color,
}
impl Colorscheme {
    pub fn from_args(args: Cli) -> Self {
        let foreground = args.color_selected_fg.into();
        let background = args.color_selected_bg.into();
        let selected = CellColor { foreground, background };

        let foreground = args.color_key_fg.into();
        let background = args.color_key_bg.into();
        let key = CellColor { foreground, background };

        let border = args.color_border.into();

        Self { selected, key, border }
    }
}
