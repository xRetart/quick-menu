#![feature(unix_sigpipe)]
#![feature(map_try_insert)]
#![warn(
     clippy::pedantic,
     clippy::all,
     clippy::cargo,
 )]

mod args;
mod events;
mod interface;
mod parse;

use {
    anyhow::Result,
    interface::{ui::Colorscheme, Ui},
    parse::MenuOption,
    events::Choice,
};

#[unix_sigpipe = "inherit"]
fn main() -> Result<()> {
    use {
        args::Cli,
        clap::Parser,
        events::event_loop,
        interface::{ui::colorscheme::TextColor, Terminal},
        parse::from_stdin,
    };

    let args = Cli::parse();

    let options = from_stdin()?;

    let colorscheme = Colorscheme {
        selected: TextColor {
            foreground: args.color_selected_fg.into(),
            background: args.color_selected_bg.into(),
        },
        unselected_display: TextColor {
            foreground: args.color_unselected_fg.into(),
            background: args.color_unselected_bg.into(),
        },
        unselected_key: TextColor {
            foreground: args.color_key_fg.into(),
            background: args.color_key_bg.into(),
        },
    };
    let ui = Ui::new(options.clone(), &colorscheme);

    let choice = Terminal::inside(|terminal| event_loop(terminal, ui, &options))??;

    print_choice(&choice, &options)
}
fn print_choice(choice: &Choice, options: &[MenuOption]) -> Result<()> {
    use std::io::{stdout, Write};

    if let Choice::Chosen(index) = choice {
        let mut stdout = stdout().lock();
        stdout.write_all(options[*index].output.as_bytes())?;
        stdout.write_all("\n".as_bytes())?;
    }
    Ok(())
}
