use std::{fs, io};

pub struct Config {
    owm_api_key: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Config, io::Error> {
        let token = fs::read_to_string(path)?;
        Ok(Config { owm_api_key: token.to_string() })
    }

    pub fn get_token(&self) -> &str {
        &self.owm_api_key
    }
}