use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use anyhow::{ensure, Result};
use clap::Parser;
use ratatui::style::Color as TuiColor;

use crate::interface::ui::list::customizations::BorderStyle;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, default_value_t = Color(TuiColor::Black))]
    pub color_selected_fg: Color,

    #[arg(long, default_value_t = Color(TuiColor::Green))]
    pub color_selected_bg: Color,

    #[arg(long, default_value_t = Color(TuiColor::Black))]
    pub color_key_fg: Color,

    #[arg(long, default_value_t = Color(TuiColor::White))]
    pub color_key_bg: Color,

    #[arg(long, default_value_t = Color(TuiColor::White))]
    pub color_border: Color,

    #[arg(long, short, value_enum, default_value_t = BorderStyle::Thick)]
    pub border_style: BorderStyle,
}

#[derive(Clone)]
pub struct Color(TuiColor);
impl Color {
    fn hex_to_rgb(hex: &str) -> Result<TuiColor> {
        let mut hex = hex.chars();
        ensure!(hex.clone().count() == 7, "six hex digits are necessary for a color");

        let hash = hex.next();
        debug_assert_eq!(hash, Some('#'));

        let mut take_segment =
            || u8::from_str_radix(hex.by_ref().take(2).collect::<String>().as_str(), 16);
        let r = take_segment()?;
        let g = take_segment()?;
        let b = take_segment()?;

        Ok(TuiColor::Rgb(r, g, b))
    }
}

impl From<Color> for TuiColor {
    fn from(value: Color) -> Self {
        value.0
    }
}
impl FromStr for Color {
    type Err = anyhow::Error;
    fn from_str(text: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(match text {
            "red" => TuiColor::Red,
            "light red" => TuiColor::LightRed,
            "blue" => TuiColor::Blue,
            "light blue" => TuiColor::LightBlue,
            "yellow" => TuiColor::Yellow,
            "light yellow" => TuiColor::LightYellow,
            "cyan" => TuiColor::Cyan,
            "light cyan" => TuiColor::LightCyan,
            "green" => TuiColor::Green,
            "light green" => TuiColor::LightGreen,
            "magenta" => TuiColor::Magenta,
            "light magenta" => TuiColor::LightMagenta,
            "gray" => TuiColor::Gray,
            "dark gray" => TuiColor::DarkGray,
            "black" => TuiColor::Black,
            "white" => TuiColor::White,
            other => {
                if other.starts_with('#') {
                    Self::hex_to_rgb(other)?
                }
                else {
                    TuiColor::Indexed(other.parse()?)
                }
            },
        }))
    }
}
impl Display for Color {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut text = |text| formatter.write_str(text);

        match self.0 {
            TuiColor::Red => text("red"),
            TuiColor::LightRed => text("light red"),
            TuiColor::Blue => text("blue"),
            TuiColor::LightBlue => text("light blue"),
            TuiColor::Yellow => text("yellow"),
            TuiColor::LightYellow => text("light yellow"),
            TuiColor::Cyan => text("cyan"),
            TuiColor::LightCyan => text("light cyan"),
            TuiColor::Green => text("green"),
            TuiColor::LightGreen => text("light green"),
            TuiColor::Magenta => text("magenta"),
            TuiColor::LightMagenta => text("light magenta"),
            TuiColor::Gray => text("gray"),
            TuiColor::DarkGray => text("dark gray"),
            TuiColor::Black => text("black"),
            TuiColor::White => text("white"),

            TuiColor::Indexed(i) => write!(formatter, "terminal color \"{i}\""),
            TuiColor::Rgb(r, g, b) => write!(formatter, "(r: {r}, g: {g}, b: {b})"),
            TuiColor::Reset => panic!("invalid internal color"),
        }
    }
}
