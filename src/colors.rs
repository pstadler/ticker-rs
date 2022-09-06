use std::{fmt, str::FromStr};

#[derive(Debug, Default, Clone)]
pub struct Color {
    s: String,
}

impl Color {
    fn from(s: &str) -> Self {
        Self { s: s.to_string() }
    }
}

impl FromStr for Color {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<String> for Color {
    fn from(s: String) -> Self {
        Self::from(&s)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // replace "\e" and "\033" with "\x1b" to not break output when supplying
        // colors via env variables (e.g. `COLOR_BOLD="\e[38;5;248m"`)
        let s = self.s.replace("\\e", "\x1b").replace("\\033", "\x1b");
        write!(f, "{}", s)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Colors {
    pub green: Color,
    pub red: Color,
    pub bold: Color,
    pub reset: Color,
    pub none: Color,
}

impl Colors {
    pub fn new(green: Option<String>, red: Option<String>, bold: Option<String>) -> Self {
        Self {
            green: Self::opt_to_color(green, "\x1b[32m"),
            red: Self::opt_to_color(red, "\x1b[31m"),
            bold: Self::opt_to_color(bold, "\x1b[1;37m"),
            reset: Self::opt_to_color(None, "\x1b[00m"),
            none: Color::default(),
        }
    }

    pub fn monochrome() -> Self {
        Self::default()
    }

    fn opt_to_color(o: Option<String>, d: &str) -> Color {
        o.unwrap_or(d.to_string()).into()
    }
}
