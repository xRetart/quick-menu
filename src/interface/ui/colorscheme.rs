pub type Color = tui::style::Color;

pub struct TextColor {
    pub foreground: Color,
    pub background: Color,
}

pub struct Colorscheme {
    pub selected: TextColor,
    pub key: TextColor,
    pub border: Color,
}
