use {
    anyhow::{anyhow, Error, Result},
    std::{
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
};

#[derive(Clone)]
pub struct MenuOption {
    pub key: char,
    pub output: String,
    pub display: String,
}
impl FromStr for MenuOption {
    type Err = Error;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        use anyhow::ensure;

        let whitespace = |c: &char| c.is_whitespace();
        let not_separator = |c: &char| c != &'|';
        let mut chars = line.chars();

        let key = chars.next().ok_or(anyhow!("Expected a key."))?;

        let mut chars = chars.skip_while(whitespace);
        ensure!(
            matches!(chars.next(), Some(':')),
            anyhow!("Expected a separator")
        );
        let mut chars = chars.skip_while(whitespace);

        let output = chars.by_ref().take_while(not_separator).collect::<String>();
        let display = chars.collect();

        Ok(Self {
            key,
            output,
            display,
        })
    }
}
impl Display for MenuOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.key, self.display)
    }
}

pub fn from_stdin() -> Result<Vec<MenuOption>> {
    use {
        anyhow::Context,
        std::io::{self, stdin, BufRead},
    };

    let parse = |line: String| {
        line.parse::<MenuOption>()
            .with_context(|| format!("Failed to parse following line from stdin: \"{}\"", line))
    };
    let parse_line = |line: io::Result<_>| {
        line.context("Reading line from stdin failed.")
            .and_then(parse)
    };

    stdin().lock().lines().map(parse_line).collect()
}
