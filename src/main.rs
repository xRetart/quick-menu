#![feature(unix_sigpipe)]
#![feature(map_try_insert)]

mod events;
mod interface;
mod parse;

use {anyhow::Result, interface::Ui, parse::MenuOption};

#[unix_sigpipe = "inherit"]
fn main() -> Result<()> {
    use {events::event_loop, interface::Terminal, parse::from_stdin};

    let options = from_stdin()?;
    let ui = Ui::with_options(options.clone());

    let choice = Terminal::inside(|terminal| event_loop(terminal, ui, &options))??;

    print_choice(choice, &options)
}
fn print_choice(choice: Option<usize>, options: &[MenuOption]) -> Result<()> {
    use std::io::{stdout, Write};

    if let Some(index) = choice {
        let mut stdout = stdout().lock();
        stdout.write_all(options[index].output.as_bytes())?;
        stdout.write_all("\n".as_bytes())?;
    }
    Ok(())
}
