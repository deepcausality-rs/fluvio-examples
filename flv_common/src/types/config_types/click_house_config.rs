use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClickHouseConfig {
    url: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

impl ClickHouseConfig {
    pub fn new(
        url: String,
        port: u16,
        username: String,
        password: String,
        database: String,
    ) -> Self {
        Self {
            url,
            port,
            username,
            password,
            database,
        }
    }
}

impl Default for ClickHouseConfig {
    fn default() -> Self {
        Self {
            url: "127.0.0.1".to_string(),
            port: 9000,
            username: "".to_string(),
            password: "".to_string(),
            database: "default".to_string(),
        }
    }
}

impl ClickHouseConfig {
    pub fn connection_string(&self) -> String {
        format!("{}:{}", self.url, self.port)
    }
}

impl ClickHouseConfig {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn database(&self) -> &str {
        &self.database
    }
}

impl fmt::Display for ClickHouseConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ClickHouseConfig {{ url: {}, port: {}, database: {}, username: {} }}",
            self.url, self.port, self.database, self.username
        )
    }
}
