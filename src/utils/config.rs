use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub email: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
        }
    }
}

impl Config {
    pub fn load_from_file() -> Self {
        if let Ok(contents) = std::fs::read_to_string("config.yml") {
            if let Ok(config) = serde_yml::from_str(&contents) {
                return config;
            }
        }
        Self::default()
    }

    pub fn save_to_file(&self) {
        if let Ok(contents) = serde_yml::to_string(self) {
            let _ = std::fs::write("config.yml", contents);
        }
    }
}
