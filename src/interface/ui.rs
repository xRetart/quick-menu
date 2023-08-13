use tui::layout::Rect;

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
    pub width: u16,
    pub height: u16,
}
impl<'l> OptionsList<'l> {
    pub fn from_options(options: &[MenuOption]) -> Self {
        use tui::style::Modifier;

        let style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Green)
            .fg(Color::DarkGray);

        let width = options.iter().map(|option| option.to_string().len()).max().unwrap_or(0).try_into().unwrap();
        let height = options.len().try_into().unwrap();

        let length = options.len();
        let items = options.iter().map(Self::make_item).collect::<Vec<_>>();

        let state = OptionsState::with_length(length);
        let list = List::new(items).highlight_style(style);
        Self { state, list, width, height }
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
        let centered = self.center_options(frame.size());
        let options = self.options.list.clone();
        let state = &mut self.options.state.state;

        frame.render_stateful_widget(options, centered, state)
    }
    fn center_options(&self, outer: Rect) -> Rect {
        let x = (outer.width - self.options.width) / 2;
        let y = (outer.height - self.options.height) / 2;
        let width = self.options.width;
        let height = self.options.height;

        Rect { x, y, width, height }
    }
}
