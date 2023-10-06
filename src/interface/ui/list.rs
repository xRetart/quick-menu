use clap::ValueEnum;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List as TuiList, ListItem, ListState},
};
use textwrap::{wrap, Options};

use super::{colorscheme::TextColor, Colorscheme};
use crate::{interface::ui::Coordinate, parse::MenuOption};

pub struct List<'l> {
    pub state: State,
    pub list: TuiList<'l>,
    pub dimensions: Coordinate,
    pub options: &'l [MenuOption],
    pub customizations: Customizations,
}

impl<'l> List<'l> {
    pub fn new(options: &'l [MenuOption], customizations: Customizations) -> Self {
        let Customizations { colorscheme: _, border_style } = customizations;

        let length = options.len();
        let state = State::with_length(length);

        let border_size = if matches!(border_style, BorderStyle::None) { 0 } else { 2 };
        let width = options_width(options);
        let height = u16::try_from(options.len()).unwrap();
        let dimensions = Coordinate { x: width + border_size, y: height + border_size };

        let list = Self::create_list(options, width, &customizations);

        Self { state, list, dimensions, options, customizations }
    }
    fn create_list(
        options: &'l [MenuOption],
        width: u16,
        Customizations { colorscheme, border_style }: &Customizations,
    ) -> TuiList<'l> {
        let border_size = if matches!(border_style, BorderStyle::None) { 0 } else { 2 };
        let style = Style::default();
        let border_color = style.fg(colorscheme.border);
        let highlight_style = style
            .add_modifier(Modifier::BOLD)
            .bg(colorscheme.selected.background)
            .fg(colorscheme.selected.foreground);

        let items = options
            .iter()
            .map(|text| Self::create_item(text, width - border_size, colorscheme.key))
            .collect::<Vec<_>>();
        let block = border_style.apply(Block::default().style(style).border_style(border_color));
        TuiList::new(items).highlight_style(highlight_style).block(block)
    }
    fn create_item(option: &'l MenuOption, width: u16, key_color: TextColor) -> ListItem<'l> {
        let MenuOption { key, output: _, display } = option;

        let default_style = Style::default();
        let display_style = default_style;
        let key_style = default_style
            .add_modifier(Modifier::BOLD)
            .fg(key_color.foreground)
            .bg(key_color.background);

        let wrap = wrap(
            display,
            Options::new(usize::try_from(width - 4).unwrap()).subsequent_indent("    "),
        );
        let mut wrapped_display = wrap.iter().map(|line| Span::styled(line.clone(), display_style));
        let mut text = Vec::with_capacity(2);
        text.push(Line::from(vec![
            Span::styled(format!(" {key} "), key_style),
            Span::styled(" ", display_style),
            wrapped_display.next().unwrap(),
        ]));
        text.extend(wrapped_display.map(|line| Line::from(vec![line])));

        ListItem::new(text)
    }
    pub fn fit(&mut self, width: u16) {
        self.list = Self::create_list(self.options, width, &self.customizations);
    }
}

fn options_width(options: &[MenuOption]) -> u16 {
    let to_lengths = |option: &MenuOption| option.to_string().chars().count();

    let key_chars = 3;
    let display_chars = options.iter().map(to_lengths).max().unwrap() + 1;

    (display_chars + key_chars).try_into().unwrap()
}

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

pub struct State {
    pub length: usize,
    pub state: ListState,
}

impl State {
    pub fn with_length(length: usize) -> Self {
        Self { length, state: ListState::default() }
    }
    pub fn next(&mut self) {
        let new = self.state.selected().map_or(0, |index| (index + 1) % self.length);
        self.state.select(Some(new));
    }
    pub fn previous(&mut self) {
        let last = self.length - 1;
        let new = self.state.selected().map_or(last, |index| index.checked_sub(1).unwrap_or(last));
        self.state.select(Some(new));
    }
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
}
