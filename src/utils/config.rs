use cookie_store::CookieStore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
    pub cookies: CookieStore
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub users: HashMap<String, UserCredentials>,
}

impl Default for Config {
    fn default() -> Self {
        Self { users: HashMap::new() }
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
        if let Ok(contents) = serde_yml::to_string(&self) {
            let _ = std::fs::write("config.yml", contents);
        }
    }
}
