use crate::error::QueryError;
use crate::QueryDBManager;
use common::prelude::TradeBar;

impl QueryDBManager {
    /// Retrieves all trade bars for the given symbol table from the database.
    ///
    /// # Arguments
    ///
    /// * `symbol_table` - The name of the symbol table to query
    ///
    /// # Returns
    ///
    /// A `Result` with a `Vec` of `TradeBar` structs if successful, or a `QueryError` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function may return the following errors:
    ///
    /// - `QueryError::QueryFailed` if the query to the DB failed.
    /// - `QueryError::EmptyTableName` if `table_name` is empty
    /// - `QueryError::InvalidTableName` if `table_name` contains invalid characters
    /// - `QueryError::TableNameTooLong` if `table_name` is longer than 64 characters
    ///
    /// See wrapped errors for more details.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///  let db_config =  DBConfig::new(9009, "0.0.0.0".into(), "exchanges".to_string());
    ///  let mut query_manager = QueryDBManager::new(db_config).await.expect("Failed to create db connection");
    ///
    ///  let trades = query_manager.get_all_trades("kraken_ethaed")
    ///               .await.expect("Failed to get all trades");
    /// }
    /// ```
    pub async fn get_all_trades(
        &mut self,
        symbol_table: &str,
    ) -> Result<Vec<TradeBar>, QueryError> {
        // Sanitize table name input to prevent SQL injection.
        let sanitized_name = match self.sanitize_table_name(symbol_table) {
            Ok(name) => name,
            Err(e) => return Err(e),
        };

        // Build the query
        let query = self.build_get_trades_query(sanitized_name);

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

        // Build the vector holding the trade bars,
        let mut trades = Vec::with_capacity(result_rows.len());

        // Iterate over the rows, convert each row to a trade bar, and add it to the vector.
        for row in &result_rows {
            let trade = TradeBar::from(row);
            trades.push(trade);
        }

        Ok(trades)
    }
}
