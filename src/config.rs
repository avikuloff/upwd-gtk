use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    uppers: String,
    lowers: String,
    digits: String,
    symbols: String,
    #[serde(rename = "default_pool")]
    pool: String,
    #[serde(rename = "default_length")]
    length: u8,
    #[serde(rename = "default_number_of_passwords")]
    count: u32,
    max_length: u8,
    #[serde(rename = "max_number_of_passwords")]
    max_count: u32,
}

impl Default for Config {
    fn default() -> Self {
        const UPPERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const LOWERS: &str = "abcdefghijklmnopqrstuvwxyz";
        const DIGITS: &str = "0123456789";
        const SYMBOLS: &str = "*&^%$#@!~";

        Config {
            uppers: UPPERS.to_owned(),
            lowers: LOWERS.to_owned(),
            digits: DIGITS.to_owned(),
            symbols: SYMBOLS.to_owned(),
            pool: String::from(UPPERS) + LOWERS + DIGITS,
            length: 12,
            count: 1,
            max_length: 32,
            max_count: 1000,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfyError> {
        confy::load("upwd-gtk")
    }

    pub fn save(&self) -> Result<(), ConfyError> {
        confy::store("upwd-gtk", self)
    }

    pub fn uppers(&self) -> &str {
        &self.uppers
    }

    pub fn lowers(&self) -> &str {
        &self.lowers
    }

    pub fn digits(&self) -> &str {
        &self.digits
    }

    pub fn symbols(&self) -> &str {
        &self.symbols
    }

    pub fn pool(&self) -> &str {
        &self.pool
    }

    pub fn length(&self) -> u8 {
        self.length
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn max_length(&self) -> u8 {
        self.max_length
    }

    pub fn max_count(&self) -> u32 {
        self.max_count
    }

    pub fn set_pool(&mut self, pool: String) {
        self.pool = pool;
    }
    pub fn set_length(&mut self, length: u8) {
        self.length = length;
    }
    pub fn set_count(&mut self, count: u32) {
        self.count = count;
    }
}
