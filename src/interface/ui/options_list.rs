use {
    super::{colorscheme::TextColor, Colorscheme},
    crate::parse::MenuOption,
    tui::{
        style::Style,
        widgets::{List, ListItem, ListState},
    },
};

pub struct OptionsList<'l> {
    pub state: OptionsState,
    pub list: List<'l>,
    pub width: u16,
    pub height: u16,
}
impl<'l> OptionsList<'l> {
    pub fn new(options: Vec<MenuOption>, colorscheme: &Colorscheme) -> Self {
        use tui::{
            style::Modifier,
            widgets::{Block, BorderType, Borders},
        };

        let style = Style::default();
        let highlight_style = style
            .add_modifier(Modifier::BOLD)
            .bg(colorscheme.selected.background)
            .fg(colorscheme.selected.foreground);

        let length = options.len();

        let state = OptionsState::with_length(length);
        let width = options
            .iter()
            .map(|option| option.to_string().len())
            .max()
            .unwrap_or(0)
            .try_into()
            .unwrap();
        let height = options.len().try_into().unwrap();

        let items = options
            .into_iter()
            .map(|text| {
                Self::make_item(
                    text,
                    width,
                    &colorscheme.unselected_display,
                    &colorscheme.unselected_key,
                )
            })
            .collect::<Vec<_>>();
        let block = Block::default()
            .style(style)
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .style(style);
        let list = List::new(items)
            .highlight_style(highlight_style)
            .block(block);

        Self {
            state,
            list,
            width,
            height,
        }
    }
    fn make_item(
        option: MenuOption,
        width: u16,
        display_color: &TextColor,
        key_color: &TextColor,
    ) -> ListItem<'l> {
        use tui::text::{Span, Spans};

        let MenuOption {
            key,
            output: _,
            display,
        } = option;

        let display_style = Style::default()
            .fg(display_color.foreground)
            .bg(display_color.background);
        let key_style = Style::default()
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

pub struct OptionsState {
    pub length: usize,
    pub state: ListState,
}
impl OptionsState {
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
