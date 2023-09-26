use {
    super::{colorscheme::TextColor, Colorscheme},
    crate::{interface::ui::Coordinate, parse::MenuOption},
    tui::{
        style::Style,
        widgets::{ListItem, ListState},
    },
};

type TuiList<'l> = tui::widgets::List<'l>;
pub struct Customizations {
    pub colorscheme: Colorscheme,
    pub noborders: bool,
}
pub struct List<'l> {
    pub state: State,
    pub list: TuiList<'l>,
    pub dimensions: Coordinate,
}
impl<'l> List<'l> {
    pub fn new(options: &'l [MenuOption], customizations: &Customizations) -> Self {
        let Customizations {
            colorscheme,
            noborders,
        } = customizations;

        let length = options.len();
        let state = State::with_length(length);

        let border_size = if customizations.noborders { 0 } else { 2 };
        let width = options_width(options);
        let height = u16::try_from(options.len()).unwrap();
        let dimensions = Coordinate {
            x: width + border_size,
            y: height + border_size,
        };

        let list = Self::create_list(options, width, colorscheme, *noborders);

        Self {
            state,
            list,
            dimensions,
        }
    }
    fn create_list(
        options: &[MenuOption],
        width: u16,
        colorscheme: &Colorscheme,
        noborders: bool,
    ) -> TuiList<'l> {
        use {
            tui::style::Modifier,
            tui::widgets::{Block, BorderType, Borders},
        };

        let noborders = if noborders {
            Borders::NONE
        } else {
            Borders::ALL
        };

        let style = Style::default();
        let border_style = style.fg(colorscheme.border);
        let highlight_style = style
            .add_modifier(Modifier::BOLD)
            .bg(colorscheme.selected.background)
            .fg(colorscheme.selected.foreground);

        let items = options
            .iter()
            .map(|text| Self::create_item(text, width, colorscheme.key))
            .collect::<Vec<_>>();
        let block = Block::default()
            .style(style)
            .border_type(BorderType::Thick)
            .borders(noborders)
            .border_style(border_style)
            .style(style);
        TuiList::new(items)
            .highlight_style(highlight_style)
            .block(block)
    }
    fn create_item(option: &MenuOption, width: u16, key_color: TextColor) -> ListItem<'l> {
        use tui::{
            style::Modifier,
            text::{Span, Spans},
        };

        let MenuOption {
            key,
            output: _,
            display,
        } = option;

        let default_style = Style::default();
        let display_style = default_style;
        let key_style = default_style
            .add_modifier(Modifier::BOLD)
            .fg(key_color.foreground)
            .bg(key_color.background);

        let display_span = Span::styled(
            format!(" {display:0$}", (width - 1) as usize),
            display_style,
        );
        let key_span = Span::styled(format!(" {key} "), key_style);

        ListItem::new(Spans::from(vec![key_span, display_span]))
    }
}
fn options_width(options: &[MenuOption]) -> u16 {
    let to_lengths = |option: &MenuOption| option.to_string().chars().count();

    let key_chars = 3;
    let display_chars = options.iter().map(to_lengths).max().unwrap() + 1;

    (display_chars + key_chars).try_into().unwrap()
}

pub struct State {
    pub length: usize,
    pub state: ListState,
}
impl State {
    pub fn with_length(length: usize) -> Self {
        Self {
            length,
            state: ListState::default(),
        }
    }
    pub fn next(&mut self) {
        let new = self
            .state
            .selected()
            .map_or(0, |index| (index + 1) % self.length);
        self.state.select(Some(new));
    }
    pub fn previous(&mut self) {
        let last = self.length - 1;
        let new = self
            .state
            .selected()
            .map_or(last, |index| index.checked_sub(1).unwrap_or(last));
        self.state.select(Some(new));
    }
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
}