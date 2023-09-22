use crate::args::Cli;

pub type Color = tui::style::Color;

#[derive(Clone, Copy)]
pub struct TextColor {
    pub foreground: Color,
    pub background: Color,
}

#[derive(Clone, Copy)]
pub struct Colorscheme {
    pub selected: TextColor,
    pub key: TextColor,
    pub border: Color,
}
impl Colorscheme {
    pub fn from_args(args: Cli) -> Self {
        Self {
            selected: TextColor {
                foreground: args.color_selected_fg.into(),
                background: args.color_selected_bg.into(),
            },
            key: TextColor {
                foreground: args.color_key_fg.into(),
                background: args.color_key_bg.into(),
            },
            border: args.color_border.into(),
        }
    }
}
