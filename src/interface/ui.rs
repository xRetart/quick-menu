use {
    crate::parse::MenuOption,
    tui::{
        backend::Backend,
        style::{Color, Style},
        widgets::{List, ListItem, ListState},
        Frame,
    },
};

pub struct OptionsState {
    length: usize,
    state: ListState,
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
        self.state.select(Some(new))
    }
    pub fn previous(&mut self) {
        let last = self.length - 1;
        let new = self
            .state
            .selected()
            .map_or(last, |index| index.checked_sub(1).unwrap_or(last));
        self.state.select(Some(new))
    }
    pub fn unselect(&mut self) {
        self.state.select(None)
    }
    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
}

pub struct OptionsList<'l> {
    pub state: OptionsState,
    pub list: List<'l>,
}
impl<'l> OptionsList<'l> {
    pub fn from_options(options: &[MenuOption]) -> Self {
        use tui::style::Modifier;
        let style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Green)
            .fg(Color::DarkGray);

        let length = options.len();
        let items = options.iter().map(Self::make_item).collect::<Vec<_>>();

        let state = OptionsState::with_length(length);
        let list = List::new(items).highlight_style(style);
        Self { state, list }
    }
    fn make_item(option: &MenuOption) -> ListItem<'l> {
        ListItem::new(option.to_string())
    }
}

pub struct Ui<'o> {
    pub options: OptionsList<'o>,
}
impl<'o> Ui<'o> {
    pub fn with_options(options: &[MenuOption]) -> Self {
        let options = OptionsList::from_options(options);
        Self { options }
    }
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        frame.render_stateful_widget(
            self.options.list.clone(),
            frame.size(),
            &mut self.options.state.state,
        )
    }
}
