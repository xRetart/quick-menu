use {
    crate::{
        interface::{Terminal, Ui},
        parse::MenuOption,
    },
    crossterm::event::KeyCode,
    crossterm::event::{self, KeyEvent},
    std::io::Result,
};

pub fn event_loop(
    terminal: &mut Terminal,
    mut ui: Ui,
    options: &[MenuOption],
) -> Result<Option<usize>> {
    let mut choice = None;
    while choice.is_none() {
        terminal.draw(|frame| ui.render(frame))?;
        choice = handle_event(&mut ui, options)?;
    }
    Ok(choice.unwrap())
}
fn handle_event(ui: &mut Ui, options: &[MenuOption]) -> Result<Option<Option<usize>>> {
    use event::{read, Event};

    match read()? {
        Event::Key(key) => Ok(handle_key(key, ui, options)),
        _ => Ok(None),
    }
}
#[allow(clippy::match_like_matches_macro)]
fn handle_key(key: KeyEvent, ui: &mut Ui, options: &[MenuOption]) -> Option<Option<usize>> {
    match key.code {
        KeyCode::Down => ui.options.state.next(),
        KeyCode::Up => ui.options.state.previous(),
        KeyCode::Left => ui.options.state.unselect(),
        KeyCode::Char(' ') => return ui.options.state.selected().map(Some),
        KeyCode::Char(key) => return map_key(key, options).map(Some),
        KeyCode::Enter => return Some(Some(ui.options.state.selected().unwrap_or(0))),
        KeyCode::Esc => return Some(None),
        _ => {}
    }
    None
}
fn map_key(key: char, options: &[MenuOption]) -> Option<usize> {
    options.iter().position(|option| option.key == key)
}
