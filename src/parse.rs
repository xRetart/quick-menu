use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, stdin, BufRead, BufReader},
    path::Path,
    str::FromStr,
};

use anyhow::{anyhow, ensure, Context, Error, Result};

pub struct MenuOption<'o, 'd> {
    pub key: char,
    pub output: Cow<'o, str>,
    pub display: Cow<'d, str>,
}
impl<'o, 'd> FromStr for MenuOption<'o, 'd> {
    type Err = Error;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let whitespace = |c: &char| c.is_whitespace();
        let not_separator = |c: &char| c != &'|';
        let mut chars = line.chars();

        let key = chars.next().ok_or_else(|| anyhow!("Expected a key."))?;

        let mut chars = chars.skip_while(whitespace);
        ensure!(matches!(chars.next(), Some(':')), anyhow!("Expected a separator"));
        let mut chars = chars.skip_while(whitespace);

        let output = chars.by_ref().take_while(not_separator).collect();
        let display = chars.collect();

        Ok(Self { key, output, display })
    }
}
impl<'o, 'd> Display for MenuOption<'o, 'd> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self { key: _, output: _, display } = self;
        write!(f, "{display}")
    }
}

pub fn from_file(path: Option<&Path>) -> Result<Box<[MenuOption<'static, 'static>]>> {
    let parse = |line: String| {
        line.parse::<MenuOption>()
            .with_context(|| format!("Failed to parse following line from stdin: \"{line}\""))
    };
    let parse_line =
        |line: io::Result<_>| line.context("Reading line from stdin failed.").and_then(parse);

    let options = match path {
        Some(path) => {
            BufReader::new(File::open(path)?).lines().map(parse_line).collect::<Result<Box<_>>>()?
        },
        None => stdin().lines().map(parse_line).collect::<Result<Box<_>>>()?,
    };
    if options.is_empty() {
        Err(anyhow!("No options where given."))
    }
    else {
        Ok(options)
    }
}
