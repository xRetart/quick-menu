use {
    std::io::{self, Result, Stderr},
    tui::{backend::CrosstermBackend, Frame},
};
pub type Backend = CrosstermBackend<Stderr>;
pub struct Terminal(tui::Terminal<Backend>);

impl Terminal {
    pub fn inside<R, F>(action: F) -> Result<R>
    where
        F: FnOnce(&mut Terminal) -> R,
    {
        let mut terminal = Self::open()?;
        let result = action(&mut terminal);
        terminal.close()?;
        Ok(result)
    }
    pub fn draw<F: FnOnce(&mut Frame<Backend>)>(&mut self, render: F) -> Result<()> {
        let Self(inner) = self;
        inner.draw(render)?;
        Ok(())
    }

    fn open() -> Result<Self> {
        use crossterm::{
            event::EnableMouseCapture,
            execute,
            terminal::{enable_raw_mode, EnterAlternateScreen},
        };
        use io::stderr;

        enable_raw_mode()?;
        let mut stderr = stderr();
        execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = Backend::new(stderr);
        let inner = tui::Terminal::new(backend)?;
        Ok(Self(inner))
    }
    fn close(self) -> Result<()> {
        use crossterm::{
            event::DisableMouseCapture,
            execute,
            terminal::{disable_raw_mode, LeaveAlternateScreen},
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
