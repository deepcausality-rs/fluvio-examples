pub mod error;
mod query_ohlcv;
mod query_symbols;
mod query_trades;
mod query_utils;
mod stream_ohlcv;
mod stream_trades;
mod types;
mod query_gen;

use common::prelude::{ClickHouseConfig};
use std::fmt::Error;
use clickhouse::Client;

const FN_NAME: &str = "[QueryDBManager]:";

pub struct QueryDBManager {
    client: Client
}

impl QueryDBManager {
    /// Creates a new QueryDBManager instance.
    ///
    /// # Arguments
    ///
    /// * `db_config: ClickHouseConfig` - The database configuration containing connection parameters.
    ///
    /// # Returns
    ///
    /// A new QueryDBManager instance connected to the database.
    ///
    /// # Errors
    ///
    /// Will return an error if the connection to the database fails.
    ///
    /// # Example
    ///
    pub async fn new(db_config: ClickHouseConfig) -> Result<Self, Error> {
        let url = db_config.connection_string();
        let client = Client::default().with_url(url);
        Ok(Self { client })
    }
}
