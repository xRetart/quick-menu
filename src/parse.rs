use {
    anyhow::{anyhow, Error, Result},
    std::{
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
};

pub struct MenuOption {
    pub key: char,
    pub value: String,
}
impl FromStr for MenuOption {
    type Err = Error;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        use anyhow::ensure;

        let whitespace = |c: &char| c.is_whitespace();
        let mut chars = line.chars();

        let key = chars.next().ok_or(anyhow!("Expected a key."))?;

        let mut chars = chars.skip_while(whitespace);
        ensure!(
            matches!(chars.next(), Some(':')),
            anyhow!("Expected a separator")
        );
        let chars = chars.skip_while(whitespace);

        let value = chars.collect();

        Ok(Self { key, value })
    }
}
impl Display for MenuOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.key, self.value)
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
