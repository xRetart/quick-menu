#![feature(unix_sigpipe)]
#![feature(map_try_insert)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod args;
mod events;
mod interface;
mod parse;

use anyhow::Result;
use args::Cli;
use events::Choice;
use interface::Ui;
use parse::MenuOption;

#[unix_sigpipe = "inherit"]
fn main() -> Result<()> {
    use clap::Parser;
    use parse::from_stdin;

    let arguments = Cli::parse();
    let options = from_stdin()?;

    let ui = create_ui(arguments, &options);
    let choice = run_ui(ui, &options)?;

    print_choice(&choice, &options)
}
fn run_ui(ui: Ui, options: &[MenuOption]) -> Result<Choice> {
    use events::event_loop;
    use interface::Terminal;

    let event_loop = |terminal: &mut _| event_loop(terminal, ui, options);
    Terminal::inside(event_loop)?
}
fn create_ui(arguments: Cli, options: &[MenuOption]) -> Ui {
    use interface::ui::{Colorscheme, Customizations};

    let border_style = arguments.border_style;
    let colorscheme = Colorscheme::from_args(arguments);
    let customizations = Customizations { colorscheme, border_style };

    Ui::new(options, customizations)
}
fn print_choice(choice: &Choice, options: &[MenuOption]) -> Result<()> {
    use std::io::{stdout, Write};

    if let Choice::Chosen(index) = choice {
        let mut stdout = stdout().lock();
        let chosen = options[*index].output.as_str();
        writeln!(stdout, "{chosen}")?;
    }
    Ok(())
}
