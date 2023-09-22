use {anyhow::Result, clap::Parser, std::fmt::Display, std::str::FromStr};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, default_value_t = Color(tui::style::Color::Black))]
    pub color_selected_fg: Color,

    #[arg(long, default_value_t = Color(tui::style::Color::Green))]
    pub color_selected_bg: Color,

    #[arg(long, default_value_t = Color(tui::style::Color::Black))]
    pub color_key_fg: Color,

    #[arg(long, default_value_t = Color(tui::style::Color::White))]
    pub color_key_bg: Color,

    #[arg(long, default_value_t = Color(tui::style::Color::White))]
    pub color_border: Color,

    #[arg(long, short, default_value_t = true)]
    pub noborders: bool,
}

#[derive(Clone)]
pub struct Color(tui::style::Color);
impl Color {
    fn hex_to_rgb(hex: &str) -> Result<tui::style::Color> {
        use anyhow::ensure;

        let mut hex = hex.chars();
        ensure!(
            hex.clone().count() == 7,
            "six hex digits are necessary for a color"
        );

        let hash = hex.next();
        debug_assert_eq!(hash, Some('#'));

        let mut take_segment =
            || u8::from_str_radix(hex.by_ref().take(2).collect::<String>().as_str(), 16);
        let r = take_segment()?;
        let g = take_segment()?;
        let b = take_segment()?;

        Ok(tui::style::Color::Rgb(r, g, b))
    }
}

impl From<Color> for tui::style::Color {
    fn from(value: Color) -> Self {
        value.0
    }
}
impl FromStr for Color {
    type Err = anyhow::Error;
    fn from_str(text: &str) -> std::result::Result<Self, Self::Err> {
        use tui::style::Color;

        Ok(Self(match text {
            "red" => Color::Red,
            "light red" => Color::LightRed,
            "blue" => Color::Blue,
            "light blue" => Color::LightBlue,
            "yellow" => Color::Yellow,
            "light yellow" => Color::LightYellow,
            "cyan" => Color::Cyan,
            "light cyan" => Color::LightCyan,
            "green" => Color::Green,
            "light green" => Color::LightGreen,
            "magenta" => Color::Magenta,
            "light magenta" => Color::LightMagenta,
            "gray" => Color::Gray,
            "dark gray" => Color::DarkGray,
            "black" => Color::Black,
            "white" => Color::White,
            other => {
                if other.starts_with('#') {
                    Self::hex_to_rgb(other)?
                } else {
                    tui::style::Color::Indexed(other.parse()?)
                }
            }
        }))
    }
}
impl Display for Color {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use tui::style::Color;
        let mut text = |text| formatter.write_str(text);

        match self.0 {
            Color::Red => text("red"),
            Color::LightRed => text("light red"),
            Color::Blue => text("blue"),
            Color::LightBlue => text("light blue"),
            Color::Yellow => text("yellow"),
            Color::LightYellow => text("light yellow"),
            Color::Cyan => text("cyan"),
            Color::LightCyan => text("light cyan"),
            Color::Green => text("green"),
            Color::LightGreen => text("light green"),
            Color::Magenta => text("magenta"),
            Color::LightMagenta => text("light magenta"),
            Color::Gray => text("gray"),
            Color::DarkGray => text("dark gray"),
            Color::Black => text("black"),
            Color::White => text("white"),

            Color::Indexed(i) => write!(formatter, "terminal color \"{i}\""),
            Color::Rgb(r, g, b) => write!(formatter, "(r: {r}, g: {g}, b: {b})"),
            Color::Reset => panic!("invalid internal color"),
        }
    }
}
