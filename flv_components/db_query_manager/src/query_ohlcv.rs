use crate::error::QueryError;
use crate::QueryDBManager;
use common::prelude::{OHLCVBar, TimeResolution};
use crate::types::OHLCVRow;
use crate::FN_NAME;

impl QueryDBManager {
    /// Retrieves all OHLCV data bars for the given symbol table and time resolution.
    ///
    /// # Parameters
    ///
    /// - `symbol_table` - The name of the symbol table to query
    /// - `time_resolution` - The time resolution to use for the query
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<DataBar>)` - The vector containing all the OHLCV data bars.
    /// - `Err(QueryError)` - If there was an error executing the query.
    ///
    /// # Errors
    ///
    /// - Returns a `QueryError` if:
    ///   - The table name could not be sanitized
    ///   - The query failed to execute
    ///
    /// # Remarks
    ///
    /// - Sanitizes the table name to prevent SQL injection.
    /// - Builds a SQL query based on the parameters.
    /// - Executes the query and converts the rows to `DataBar` objects.
    /// - Returns an empty vector if there are no results.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::{DBConfig, TimeResolution};
    /// use db_query_manager::QueryDBManager;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// use common::prelude::ClickHouseConfig;
    /// let db_config =  ClickHouseConfig::default();
    ///  let mut query_manager = QueryDBManager::new(db_config).await.expect("Failed to create db connection");
    ///
    ///  let time_resolution = TimeResolution::FiveMin;
    ///  let trades = query_manager.get_all_ohlcv_bars(278, "kraken_ethaed", &time_resolution)
    ///               .await.expect("Failed to get all trades");
    ///
    /// }
    /// ```
    pub async fn get_all_ohlcv_bars(
        &mut self,
        symbol_id: u16,
        symbol_table: &str,
        time_resolution: &TimeResolution,
    ) -> Result<Vec<OHLCVBar>, QueryError> {
        // Sanitize table name input to prevent SQL injection.
        let sanitized_name =  self.sanitize_table_name(symbol_table).expect("Failed to sanitize table name");

        // Build the query
        let query = self.build_get_ohlcv_bars_query(sanitized_name, time_resolution);

        // Execute query
        let ohlcv_rows = self
            .client
            .query(&query)
            .fetch_all::<OHLCVRow>()
            .await
            .expect(format!("{} Failed to execute query: {}", FN_NAME, query).as_str());

        // Check for empty result
        if ohlcv_rows.is_empty() {
            return Ok(Vec::new());
        }

        let mut bars = Vec::with_capacity(ohlcv_rows.len());

        for row in ohlcv_rows {
            let bar = OHLCVBar::new(symbol_id, row.date_time(), row.open(), row.high(), row.low(), row.close(), row.volume());
            bars.push(bar);
        }

        Ok(bars)
    }
}
