use std::{env::temp_dir, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub cookies: String,
    pub crumb: String,
    path: PathBuf,
}

impl Session {
    pub fn load() -> Self {
        let mut path = temp_dir();
        path.push(format!("{}-{}", "ticker-rs", whoami::username()));

        if path.exists() {
            return serde_json::from_str(&fs::read_to_string(path).unwrap_or_default()).unwrap();
        }

        Self {
            cookies: "".to_string(),
            crumb: "".to_string(),
            path,
        }
    }

    pub fn persist(&mut self) -> Result<(), String> {
        match fs::write(&self.path, serde_json::to_string(self).unwrap()) {
            Ok(_) => (),
            Err(err) => return Err(format!("Error writing temporary file: {}", err)),
        }

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.crumb.is_empty() || self.cookies.is_empty()
    }
}
