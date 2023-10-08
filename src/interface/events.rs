use std::io::{stdout, Write};

use anyhow::{Context, Result};
use crossterm::event::{self, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use event::{read, Event, KeyCode, KeyModifiers};

use super::ui::InputMode;
use crate::{
    interface::{ui::Vector, Terminal, Ui},
    parse::MenuOption,
    Config,
};

pub enum Choice {
    Chosen(usize),
    None,
}

impl Choice {
    pub fn print(&self, options: &[MenuOption]) -> Result<()> {
        if let Self::Chosen(index) = self {
            let mut stdout = stdout().lock();
            let chosen = &options[*index].output;
            writeln!(stdout, "{chosen}")?;
        }
        Ok(())
    }
}
pub fn event_loop(
    terminal: &mut Terminal,
    mut ui: Ui,
    options: &[MenuOption],
    config: &Config,
) -> Result<Choice> {
    let mut choice = None;
    while choice.is_none() {
        terminal
            .draw(|frame| ui.render(frame))
            .context("Drawing the rendered inteface to the terminal failed.")?;
        choice = handle_event(&mut ui, options, config)
            .context("Handling the incoming event failed.")?;
    }
    Ok(choice.unwrap())
}
fn handle_event(ui: &mut Ui, options: &[MenuOption], config: &Config) -> Result<Option<Choice>> {
    match read().context("Reading event from backend failed.")? {
        Event::Key(key) => Ok(handle_key(key, ui, options, config)),
        Event::Mouse(mouse) => Ok(handle_mouse(mouse, ui).map(Choice::Chosen)),
        _ => Ok(None),
    }
}
fn handle_mouse(mouse: MouseEvent, ui: &mut Ui) -> Option<usize> {
    let x = mouse.column;
    let y = mouse.row;
    let coordinate = Vector { x, y };

    match mouse.kind {
        MouseEventKind::ScrollUp => ui.list.state.previous(),
        MouseEventKind::ScrollDown => ui.list.state.next(),
        MouseEventKind::Down(MouseButton::Middle) => return ui.list.state.selected(),
        MouseEventKind::Down(MouseButton::Left) => return ui.list.select(coordinate),
        _ => {},
    }
    None
}
fn handle_key(
    key: KeyEvent,
    ui: &mut Ui,
    options: &[MenuOption],
    config: &Config,
) -> Option<Choice> {
    match key.modifiers {
        KeyModifiers::NONE => match key.code {
            KeyCode::Down => ui.list.state.next(),
            KeyCode::Up => ui.list.state.previous(),
            KeyCode::Left => ui.list.state.unselect(),
            KeyCode::Char(character) => match ui.input_mode {
                InputMode::Searching => ui.append_query(character),
                InputMode::Selecting => return map_char(character, options).map(Choice::Chosen),
            },
            KeyCode::Backspace => {
                if matches!(ui.input_mode, InputMode::Searching) {
                    ui.pop_query();
                }
            },
            KeyCode::Enter => return Some(Choice::Chosen(ui.list.state.selected().unwrap_or(0))),
            KeyCode::Esc => return Some(Choice::None),
            _ => {},
        },
        KeyModifiers::CONTROL => match key.code {
            KeyCode::Char(c) if c == config.down_key => ui.list.state.previous(),
            KeyCode::Char(c) if c == config.up_key => ui.list.state.next(),
            KeyCode::Char(c) if c == config.search_key => ui.input_mode.switch(),
            _ => (),
        },
        _ => {},
    }
    None
}
fn map_char(key: char, options: &[MenuOption]) -> Option<usize> {
    options.iter().position(|option| option.key == key)
}
