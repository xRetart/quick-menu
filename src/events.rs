use {
    crate::{
        interface::{Terminal, Ui},
        parse::MenuOption,
    },
    anyhow::{Context, Result},
    crossterm::event::{self, KeyEvent, MouseEvent},
};

pub enum Choice {
    Chosen(usize),
    None,
}

pub fn event_loop(terminal: &mut Terminal, mut ui: Ui, options: &[MenuOption]) -> Result<Choice> {
    let mut choice = None;
    while choice.is_none() {
        terminal
            .draw(|frame| ui.render(frame))
            .context("Drawing the rendered inteface to the terminal failed.")?;
        choice = handle_event(&mut ui, options).context("Handling the incoming event failed.")?;
    }
    Ok(choice.unwrap())
}
fn handle_event(ui: &mut Ui, options: &[MenuOption]) -> Result<Option<Choice>> {
    use event::{read, Event};

    match read().context("Reading event from backend failed.")? {
        Event::Key(key) => Ok(handle_key(key, ui, options)),
        Event::Mouse(mouse) => Ok(handle_mouse(mouse, ui).map(Choice::Chosen)),
        _ => Ok(None),
    }
}
fn handle_mouse(mouse: MouseEvent, ui: &mut Ui) -> Option<usize> {
    use crossterm::event::{MouseButton, MouseEventKind};
    match mouse.kind {
        MouseEventKind::ScrollUp => ui.options.state.previous(),
        MouseEventKind::ScrollDown => ui.options.state.next(),
        MouseEventKind::Down(MouseButton::Middle) => return ui.options.state.selected(),
        MouseEventKind::Down(MouseButton::Left) => return ui.select(mouse.row, mouse.column),
        _ => {}
    }
    None
}
fn handle_key(key: KeyEvent, ui: &mut Ui, options: &[MenuOption]) -> Option<Choice> {
    use event::{KeyCode, KeyModifiers};
    match key.modifiers {
        KeyModifiers::NONE => match key.code {
            KeyCode::Down => ui.options.state.next(),
            KeyCode::Up => ui.options.state.previous(),
            KeyCode::Left => ui.options.state.unselect(),
            KeyCode::Char(' ') | KeyCode::Right => {
                return ui.options.state.selected().map(Choice::Chosen)
            }
            KeyCode::Char(key) => return map_key(key, options).map(Choice::Chosen),
            KeyCode::Enter => {
                return Some(Choice::Chosen(ui.options.state.selected().unwrap_or(0)))
            }
            KeyCode::Esc => return Some(Choice::None),
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
