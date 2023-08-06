use std::{
    fmt::{self, Display, Formatter},
    io,
    str::FromStr,
};

pub struct MenuOption {
    pub key: char,
    pub value: String,
}
impl FromStr for MenuOption {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let whitespace = |c: &char| c.is_whitespace();
        let mut chars = line.chars();

        let key = chars.next().ok_or("key not vailable")?;

        let mut chars = chars.skip_while(whitespace);
        (matches!(chars.next(), Some(':')))
            .then_some(())
            .ok_or("no seperator")?;
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

pub fn from_stdin() -> io::Result<Vec<MenuOption>> {
    use io::{stdin, BufRead};

    let parse = |line: String| line.parse::<MenuOption>().unwrap();

    stdin().lock().lines().map(|line| line.map(parse)).collect()
}
