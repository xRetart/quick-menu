use ratatui::widgets::ListState;

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
