use clap::ValueEnum;
use ratatui::{
    style::Style,
    widgets::{Block, BorderType, Borders},
};

use crate::interface::ui::Colorscheme;

#[derive(Clone)]
pub struct Customizations {
    pub colorscheme: Colorscheme,
    pub border_style: BorderStyle,
}
impl Customizations {
    pub fn borders<'b>(&self, block: Block<'b>) -> Block<'b> {
        self.border_style.apply(block).border_style(Style::default().fg(self.colorscheme.border))
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum BorderStyle {
    None,
    Plain,
    Thick,
    Rounded,
    Double,
}

impl BorderStyle {
    pub const fn apply(self, block: Block<'_>) -> Block<'_> {
        match self {
            Self::None => block.borders(Borders::NONE),
            Self::Plain => block.borders(Borders::ALL).border_type(BorderType::Plain),
            Self::Thick => block.borders(Borders::ALL).border_type(BorderType::Thick),
            Self::Rounded => block.borders(Borders::ALL).border_type(BorderType::Rounded),
            Self::Double => block.borders(Borders::ALL).border_type(BorderType::Double),
        }
    }
    pub const fn size(self) -> u16 {
        if matches!(self, Self::None) {
            0
        }
        else {
            2
        }
    }
}
