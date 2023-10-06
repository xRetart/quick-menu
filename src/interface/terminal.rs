use std::io::{self, Stderr};

use anyhow::{Context, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use io::stderr;
use ratatui::{backend::CrosstermBackend, Frame, Terminal as TuiTerminal};

pub type Backend = CrosstermBackend<Stderr>;
pub struct Terminal(TuiTerminal<Backend>);

impl Terminal {
    pub fn inside<R, F>(action: F) -> Result<R>
    where F: FnOnce(&mut Self) -> R {
        let mut terminal = Self::open().context("Opening terminal failed.")?;
        let result = action(&mut terminal);
        terminal.close().context("Closing terminal failed.")?;
        Ok(result)
    }
    pub fn draw<F: FnOnce(&mut Frame<Backend>)>(&mut self, render: F) -> Result<()> {
        let Self(inner) = self;
        inner.draw(render)?;
        Ok(())
    }

    fn open() -> Result<Self> {
        enable_raw_mode().context("Changing terminal mode to raw failed.")?;
        let mut stderr = stderr();
        execute!(stderr, EnterAlternateScreen, EnableMouseCapture,)
            .context("Entering alternate screen failed.")?;
        let backend = Backend::new(stderr);
        let inner = TuiTerminal::new(backend).context("Creating internal tui terminal failed.")?;
        Ok(Self(inner))
    }
    fn close(self) -> Result<()> {
        let Self(mut inner) = self;
        disable_raw_mode().context("Changing terminal mode from raw failed.")?;
        execute!(inner.backend_mut(), LeaveAlternateScreen, DisableMouseCapture,)
            .context("Leaving alternate screen failed.")?;
        inner.show_cursor().context("Showing cursor failed.")?;
        Ok(())
    }
}
