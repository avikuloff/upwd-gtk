use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "default_pool")]
    pool: String,
    #[serde(rename = "default_length")]
    length: u8,
    #[serde(rename = "default_number_of_passwords")]
    count: u32,
    max_length: u8,
    #[serde(rename = "max_number_of_passwords")]
    max_count: u32,
    pool_options: Vec<PoolOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolOption {
    name: String,
    value: String,
    checked: bool,
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    pool_options: Option<Vec<PoolOption>>,
    pool: Option<String>,
    length: Option<u8>,
    count: Option<u32>,
    max_length: Option<u8>,
    max_count: Option<u32>,
}

impl PoolOption {
    pub fn new(name: &str, value: &str, checked: bool) -> Self {
        PoolOption {
            name: name.to_owned(),
            value: value.to_owned(),
            checked,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn checked(&self) -> bool {
        self.checked
    }
}

impl Default for Config {
    fn default() -> Self {
        const UPPERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const LOWERS: &str = "abcdefghijklmnopqrstuvwxyz";
        const DIGITS: &str = "0123456789";
        const SYMBOLS: &str = "*&^%$#@!~";

        let mut pool_options = Vec::with_capacity(4);
        pool_options.push(PoolOption::new("Use UPPERCASE letters", UPPERS, true));
        pool_options.push(PoolOption::new("Use lowercase letters", LOWERS, true));
        pool_options.push(PoolOption::new("Use digits", DIGITS, true));
        pool_options.push(PoolOption::new("Use symbols", SYMBOLS, false));

        Config {
            pool: String::from(UPPERS) + LOWERS + DIGITS,
            length: 12,
            count: 1,
            max_length: 32,
            max_count: 1000,
            pool_options,
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

    pub fn pool_options(&self) -> &Vec<PoolOption> {
        &self.pool_options
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
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Config {
        let mut config = Config::default();

        if let Some(pool_options) = self.pool_options {
            config.pool_options = pool_options;
        }
        if let Some(pool) = self.pool {
            config.pool = pool;
        }
        if let Some(length) = self.length {
            config.length = length;
        }
        if let Some(count) = self.count {
            config.count = count;
        }
        if let Some(max_length) = self.max_length {
            config.max_length = max_length;
        }
        if let Some(max_count) = self.max_count {
            config.max_count = max_count;
        }

        config
    }

    pub fn pool_options(mut self, pool_options: Vec<PoolOption>) -> Self {
        self.pool_options = Some(pool_options);
        self
    }

    pub fn pool(mut self, pool: String) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn length(mut self, length: u8) -> Self {
        self.length = Some(length);
        self
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    pub fn max_length(mut self, max_length: u8) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn max_count(mut self, max_count: u32) -> Self {
        self.max_count = Some(max_count);
        self
    }
}
