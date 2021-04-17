use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    uppers: String,
    lowers: String,
    digits: String,
    symbols: String,
    use_uppers: bool,
    use_lowers: bool,
    use_digits: bool,
    use_symbols: bool,
    pool: String,
    length: u8,
    count: u32,
}

impl Default for Config {
    fn default() -> Self {
        const UPPERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const LOWERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
        const DIGITS: &'static str = "0123456789";
        const SYMBOLS: &'static str = "*&^%$#@!~";

        Config {
            uppers: UPPERS.to_owned(),
            lowers: LOWERS.to_owned(),
            digits: DIGITS.to_owned(),
            symbols: SYMBOLS.to_owned(),
            use_uppers: true,
            use_lowers: true,
            use_digits: true,
            use_symbols: false,
            pool: String::from(UPPERS) + LOWERS + DIGITS,
            length: 12,
            count: 1,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        confy::load("upwd-gtk").unwrap_or_default()
    }

    pub fn save(&self) {
        confy::store("upwd-gtk", self).unwrap();
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

    pub fn use_uppers(&self) -> bool {
        self.use_uppers
    }

    pub fn use_lowers(&self) -> bool {
        self.use_lowers
    }

    pub fn use_digits(&self) -> bool {
        self.use_digits
    }

    pub fn use_symbols(&self) -> bool {
        self.use_symbols
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
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    uppers: Option<String>,
    lowers: Option<String>,
    digits: Option<String>,
    symbols: Option<String>,
    use_uppers: Option<bool>,
    use_lowers: Option<bool>,
    use_digits: Option<bool>,
    use_symbols: Option<bool>,
    pool: Option<String>,
    length: Option<u8>,
    count: Option<u32>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Config {
        let mut config = Config::default();

        if let Some(uppers) = self.uppers {
            config.uppers = uppers;
        }
        if let Some(lowers) = self.lowers {
            config.lowers = lowers;
        }
        if let Some(digits) = self.digits {
            config.digits = digits;
        }
        if let Some(symbols) = self.symbols {
            config.symbols = symbols;
        }
        if let Some(use_uppers) = self.use_uppers {
            config.use_uppers = use_uppers;
        }
        if let Some(use_lowers) = self.use_lowers {
            config.use_lowers = use_lowers;
        }
        if let Some(use_digits) = self.use_digits {
            config.use_digits = use_digits;
        }
        if let Some(use_symbols) = self.use_symbols {
            config.use_symbols = use_symbols;
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

        config
    }

    pub fn uppers(mut self, uppers: String) -> Self {
        self.uppers = Some(uppers);
        self
    }

    pub fn lowers(mut self, lowers: String) -> Self {
        self.lowers = Some(lowers);
        self
    }

    pub fn digits(mut self, digits: String) -> Self {
        self.digits = Some(digits);
        self
    }

    pub fn symbols(mut self, symbols: String) -> Self {
        self.symbols = Some(symbols);
        self
    }

    pub fn use_uppers(mut self, use_uppers: bool) -> Self {
        self.use_uppers = Some(use_uppers);
        self
    }

    pub fn use_lowers(mut self, use_lowers: bool) -> Self {
        self.use_lowers = Some(use_lowers);
        self
    }

    pub fn use_digits(mut self, use_digits: bool) -> Self {
        self.use_digits = Some(use_digits);
        self
    }

    pub fn use_symbols(mut self, use_symbols: bool) -> Self {
        self.use_symbols = Some(use_symbols);
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
}
