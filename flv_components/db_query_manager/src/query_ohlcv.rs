use crate::error::QueryError;
use crate::QueryDBManager;
use common::prelude::{OHLCVBar, TimeResolution};

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
    pub async fn get_all_ohlcv_bars(
        &mut self,
        symbol_table: &str,
        time_resolution: &TimeResolution,
    ) -> Result<Vec<OHLCVBar>, QueryError> {
        // Sanitize table name input to prevent SQL injection.
        let sanitized_name = match self.sanitize_table_name(symbol_table) {
            Ok(name) => name,
            Err(e) => return Err(e),
        };

        // Build the query
        let query = self.build_get_ohlcv_bars_query(sanitized_name, time_resolution);

        // Execute query
        let result = self.query(&query).await;

        // Handle query errors
        let result_rows = match result {
            Ok(rows) => rows,
            Err(e) => return Err(QueryError::QueryFailed(e)),
        };

        // Check for empty result
        if result_rows.is_empty() {
            return Ok(Vec::new());
        }

        // Build the vector holding the OHLCV data bars,
        let mut trades = Vec::with_capacity(result_rows.len());

        // Iterate over the rows, convert each row to a data bar, and add the bar to the vector.
        for row in result_rows {
            let trade_bar = OHLCVBar::from(&row);
            trades.push(trade_bar);
        }

        Ok(trades)
    }
}
