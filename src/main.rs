#![feature(unix_sigpipe)]
#![feature(map_try_insert)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod args;
mod interface;
mod parse;

use anyhow::Result;
use args::Cli;
use clap::Parser;
use interface::{
    events::{event_loop, Choice},
    ui::{Colorscheme, Customizations},
    Terminal,
    Ui,
};
use parse::{from_stdin, MenuOption};

#[unix_sigpipe = "inherit"]
fn main() -> Result<()> {
    let arguments = Cli::parse();
    let options = from_stdin()?;

    let ui = create_ui(arguments, &options);
    let choice = run_ui(ui, &options)?;

    choice.print(&options)
}
fn run_ui(ui: Ui, options: &[MenuOption]) -> Result<Choice> {
    let event_loop = |terminal: &mut _| event_loop(terminal, ui, options);
    Terminal::inside(event_loop)?
}
fn create_ui(arguments: Cli, options: &[MenuOption]) -> Ui {
    let border_style = arguments.border_style;
    let colorscheme = Colorscheme::from_args(arguments);
    let customizations = Customizations { colorscheme, border_style };

    Ui::new(options, customizations)
}
