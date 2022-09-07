use std::env;

#[derive(Default)]
pub struct Colors {
    pub green: String,
    pub red: String,
    pub bold: String,
    pub reset: String,
    pub none: String,
}

pub fn get_colors() -> Colors {
    if env::var("NO_COLOR").is_ok() {
        return Colors::default();
    }

    Colors {
        green: from_env_or_default("COLOR_GREEN", "\x1b[32m"),
        red: from_env_or_default("COLOR_RED", "\x1b[31m"),
        bold: from_env_or_default("COLOR_BOLD", "\x1b[1;37m"),
        reset: "\x1b[00m".to_string(),
        none: "".to_string(),
    }
}

fn from_env_or_default(env: &str, default: &str) -> String {
    match env::var(env) {
        Ok(v) => v.replace("\\e", "\x1b").replace("\\033", "\x1b"),
        _ => default.to_string(),
    }
}
