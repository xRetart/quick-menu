use ratatui::widgets::ListState;

pub struct State {
    pub length: usize,
    pub inner: ListState,
}

impl State {
    pub fn with_length(length: usize) -> Self {
        Self { length, inner: ListState::default() }
    }
    pub fn next(&mut self) {
        let new = self.inner.selected().map_or(0, |index| (index + 1) % self.length);
        self.inner.select(Some(new));
    }
    pub fn previous(&mut self) {
        let last = self.length - 1;
        let new = self.inner.selected().map_or(last, |index| index.checked_sub(1).unwrap_or(last));
        self.inner.select(Some(new));
    }
    pub fn unselect(&mut self) {
        self.inner.select(None);
    }
    pub fn selected(&self) -> Option<usize> {
        self.inner.selected()
    }
}
