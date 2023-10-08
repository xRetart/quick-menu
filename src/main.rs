#![feature(unix_sigpipe)]
#![feature(map_try_insert)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod args;
mod config;
mod interface;
mod parse;

use anyhow::Result;
use args::Cli;
use clap::Parser;
pub use config::Config;
use interface::{
    events::{event_loop, Choice},
    ui::{Colorscheme, Customizations},
    Terminal,
    Ui,
};
use parse::{from_file, MenuOption};
const PROGRAM_NAME: &str = "quick-menu";

#[unix_sigpipe = "inherit"]
fn main() -> Result<()> {
    let configuration = confy::load::<Config>(PROGRAM_NAME, None)?;
    let arguments = Cli::parse();
    let options = from_file(arguments.options_file.as_deref())?;

    let ui = create_ui(arguments, &options);
    let choice = run_ui(ui, &options, &configuration)?;

    choice.print(&options)
}
fn run_ui(ui: Ui, options: &[MenuOption], configuration: &Config) -> Result<Choice> {
    let event_loop = |terminal: &mut _| event_loop(terminal, ui, options, configuration);
    Terminal::inside(event_loop)?
}
fn create_ui<'o>(arguments: Cli, options: &'o [MenuOption<'static, 'static>]) -> Ui<'o> {
    let border_style = arguments.border_style;
    let colorscheme = Colorscheme::from_args(arguments);
    let customizations = Customizations { colorscheme, border_style };

    Ui::new(options, customizations)
}
