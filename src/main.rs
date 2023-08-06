#![feature(unix_sigpipe)]
#![feature(map_try_insert)]

mod events;
mod interface;
mod parse;

use {
    interface::Ui,
    parse::MenuOption,
    std::io::{self, Result},
};

#[unix_sigpipe = "inherit"]
fn main() -> Result<()> {
    use {events::event_loop, interface::Terminal, parse::from_stdin};

    let options = from_stdin()?;
    let ui = Ui::with_options(&options);

    let choice = Terminal::inside(|terminal| event_loop(terminal, ui, &options))??;

    print_choice(choice, &options)
}
fn print_choice(choice: Option<usize>, options: &[MenuOption]) -> Result<()> {
    use io::{stdout, Write};

    if let Some(index) = choice {
        let mut stdout = stdout().lock();
        stdout.write_all(options[index].value.as_bytes())?;
        stdout.write_all("\n".as_bytes())?;
    }
    Ok(())
}
