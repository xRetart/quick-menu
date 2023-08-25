pub type Color = tui::style::Color;

pub struct TextColor {
    pub foreground: Color,
    pub background: Color,
}

pub struct Colorscheme {
    pub selected: TextColor,
    pub unselected_display: TextColor,
    pub unselected_key: TextColor,
    pub border: Color,
}
