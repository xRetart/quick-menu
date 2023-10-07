use clap::ValueEnum;
use ratatui::widgets::{Block, BorderType, Borders};

use crate::interface::ui::Colorscheme;

pub struct Customizations {
    pub colorscheme: Colorscheme,
    pub border_style: BorderStyle,
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
}
