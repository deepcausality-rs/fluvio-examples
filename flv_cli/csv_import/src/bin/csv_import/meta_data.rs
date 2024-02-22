use proton_client::prelude::Row;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct MetaData<'l> {
    table_name: &'l str,
    symbol: &'l str,
    symbol_id: u64,
    number_of_rows: u64,
}

impl<'l> MetaData<'l> {
    pub fn new(table_name: &'l str, symbol: &'l str, symbol_id: u64, number_of_rows: u64) -> Self {
        Self {
            table_name,
            symbol,
            symbol_id,
            number_of_rows,
        }
    }
}

impl<'l> fmt::Display for MetaData<'l> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MetaData {{ table_name: {}, symbol: {}, symbol_id: {}, number_of_rows: {} }}",
            self.table_name, self.symbol, self.symbol_id, self.number_of_rows
        )
    }
}
