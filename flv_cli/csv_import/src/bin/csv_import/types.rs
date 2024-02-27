use klickhouse::Row;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct CountRow {
    count: u64,
}

impl CountRow {
    pub fn count(&self) -> u64 {
        self.count
    }
}

#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct MetaData {
    table_name: String,
    symbol: String,
    symbol_id: u32,
    number_of_rows: u64,
}

impl MetaData {
    pub fn new(table_name: String, symbol: String, symbol_id: u32, number_of_rows: u64) -> Self {
        Self {
            table_name,
            symbol,
            symbol_id,
            number_of_rows,
        }
    }
}

impl MetaData {
    pub fn table_name(&self) -> &str {
        &self.table_name
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn symbol_id(&self) -> u32 {
        self.symbol_id
    }
    pub fn number_of_rows(&self) -> u64 {
        self.number_of_rows
    }
}

impl fmt::Display for MetaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MetaData {{ table_name: {}, symbol: {}, symbol_id: {}, number_of_rows: {} }}",
            self.table_name, self.symbol, self.symbol_id, self.number_of_rows
        )
    }
}
