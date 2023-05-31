use std::{env::temp_dir, fs};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub cookies: String,
    pub crumb: String,
}

impl Session {
    const FILE_NAME: &str = "ticker-rs";

    pub fn load() -> Self {
        let mut path = temp_dir();
        path.push(Session::FILE_NAME);

        if path.exists() {
            return serde_json::from_str(&fs::read_to_string(path).unwrap_or_default()).unwrap();
        }

        Self {
            cookies: "".to_string(),
            crumb: "".to_string(),
        }
    }

    pub fn persist(&mut self) -> Result<(), String> {
        let mut path = temp_dir();
        path.push(Session::FILE_NAME);

        match fs::write(path, serde_json::to_string(self).unwrap()) {
            Ok(_) => (),
            Err(err) => return Err(format!("Error writing temporary file: {}", err)),
        }

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.crumb.is_empty() || self.cookies.is_empty()
    }
}
