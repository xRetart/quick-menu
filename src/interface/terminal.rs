use {
    std::io::{self, Result, Stderr},
    tui::{Frame, backend::CrosstermBackend}
};
pub type Backend = CrosstermBackend<Stderr>;
pub struct Terminal(tui::Terminal<Backend>);

impl Terminal {
    pub fn open() -> Result<Self> {
        use io::stderr;
        use crossterm::{
            terminal::{enable_raw_mode, EnterAlternateScreen},
            event::EnableMouseCapture,
            execute,
        };

        enable_raw_mode()?;
        let mut stderr = stderr();
        execute!(
            stderr,
            EnterAlternateScreen,
            EnableMouseCapture
        )?;
        let backend = Backend::new(stderr);
        let inner = tui::Terminal::new(backend)?;
        Ok(Self(inner))
    }
    pub fn draw<F: FnOnce(&mut Frame<Backend>)>(&mut self, render: F) -> Result<()> {
        let Self(inner) = self;
        inner.draw(render)?;
        Ok(())
    }
    pub fn close(self) -> Result<()> {
        use crossterm::{
            terminal::{disable_raw_mode, LeaveAlternateScreen},
            execute, event::DisableMouseCapture
        };

        let Self(mut inner) = self;
        disable_raw_mode()?;
        execute!(
            inner.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        inner.show_cursor()?;
        Ok(())
    }
}
