use {
    crate::{
        interface::{Terminal, Ui},
        parse::MenuOption,
    },
    anyhow::{Context, Result},
    crossterm::event::{self, KeyEvent},
};

pub fn event_loop(
    terminal: &mut Terminal,
    mut ui: Ui,
    options: &[MenuOption],
) -> Result<Option<usize>> {
    let mut choice = None;
    while choice.is_none() {
        terminal
            .draw(|frame| ui.render(frame))
            .context("Drawing the rendered inteface to the terminal failed.")?;
        choice = handle_event(&mut ui, options).context("Handling the incoming event failed.")?;
    }
    Ok(choice.unwrap())
}
fn handle_event(ui: &mut Ui, options: &[MenuOption]) -> Result<Option<Option<usize>>> {
    use event::{read, Event};

    match read().context("Reading event from backend failed.")? {
        Event::Key(key) => Ok(handle_key(key, ui, options)),
        _ => Ok(None),
    }
}
fn handle_key(key: KeyEvent, ui: &mut Ui, options: &[MenuOption]) -> Option<Option<usize>> {
    use event::{KeyCode, KeyModifiers};
    match key.modifiers {
        KeyModifiers::NONE => match key.code {
            KeyCode::Down => ui.options.state.next(),
            KeyCode::Up => ui.options.state.previous(),
            KeyCode::Left => ui.options.state.unselect(),
            KeyCode::Char(' ') | KeyCode::Right => return ui.options.state.selected().map(Some),
            KeyCode::Char(key) => return map_key(key, options).map(Some),
            KeyCode::Enter => return Some(Some(ui.options.state.selected().unwrap_or(0))),
            KeyCode::Esc => return Some(None),
            _ => {}
        },
        KeyModifiers::CONTROL => match key.code {
            KeyCode::Char('n') => ui.options.state.next(),
            KeyCode::Char('e') => ui.options.state.previous(),
            _ => (),
        },
        _ => {}
    }
    None
}
fn map_key(key: char, options: &[MenuOption]) -> Option<usize> {
    options.iter().position(|option| option.key == key)
}
