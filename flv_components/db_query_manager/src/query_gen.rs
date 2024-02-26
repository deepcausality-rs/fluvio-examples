use common::prelude::TimeResolution;

impl crate::QueryDBManager {
    /// Builds a SQL query to get all symbol IDs and symbols from a symbol table.
    ///
    /// # Arguments
    ///
    /// * `symbol_table` - The name of the symbol table to query
    ///
    /// # Returns
    ///
    /// Returns a SQL query string to retrieve all symbol IDs and symbols from the given symbol table.
    ///
    pub fn build_get_symbol_id_query(&self, symbol_table: &str) -> String {
        format!("SELECT symbol_id, symbol FROM {}", symbol_table)
    }

    /// Builds a SQL query to get OHLCV bars from a trade table at a given time resolution.
    ///
    /// # Arguments
    ///
    /// * `trade_table` - The name of the trade table to query
    /// * `time_resolution` - The time resolution to resample the trades to
    ///
    /// # Returns
    ///
    /// Returns a SQL query string to retrieve OHLCV bars from the trade table resampled to the time resolution.
    ///
    pub fn build_get_ohlcv_bars_query(
        &self,
        trade_table: &str,
        time_resolution: &TimeResolution,
    ) -> String {
        format!(
            r"SELECT toUnixTimestamp(toStartOfInterval(timestamp, INTERVAL {time_resolution})) AS datetime,
              argMin(price, timestamp) AS open,
              max(price) AS high,
              min(price) AS low,
              argMax(price, timestamp) AS close,
              sum(volume) AS volume

            FROM {trade_table}
            GROUP BY datetime
            ORDER BY datetime"
        ).to_string()
    }

    /// Builds a SQL query to get all trades from a trade table.
    ///
    /// # Arguments
    ///
    /// * `trade_table` - The name of the trade table to query
    ///
    /// # Returns
    ///
    /// Returns a SQL query string to retrieve all timestamps, prices, and volumes from the given trade table.
    ///
    pub fn build_get_trades_query(&self, trade_table: &str) -> String {
        format!("SELECT timestamp, price, volume FROM {}", trade_table)
    }
}
