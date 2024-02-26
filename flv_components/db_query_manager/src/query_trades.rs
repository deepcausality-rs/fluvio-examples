use crate::error::QueryError;
use crate::types::TradeRow;
use crate::{QueryDBManager, FN_NAME};
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
    ///  use common::prelude::ClickHouseConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let db_config =  ClickHouseConfig::default();
    ///  let mut query_manager = QueryDBManager::new(db_config).await.expect("Failed to create db connection");
    ///
    ///  let trades = query_manager.get_all_trades(278, "kraken_ethaed")
    ///               .await.expect("Failed to get all trades");
    ///
    /// }
    /// ```
    pub async fn get_all_trades(
        &mut self,
        symbol_id: u16,
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
        let trade_rows = self
            .client
            .query_collect::<TradeRow>(&query)
            .await
            .expect(format!("{} Failed to execute query: {}", FN_NAME, query).as_str());

        // Check for empty result
        if trade_rows.is_empty() {
            return Ok(Vec::new());
        }

        let mut trades = Vec::with_capacity(trade_rows.len());

        for row in trade_rows {
            let bar = TradeBar::new(symbol_id, row.date_time(), row.price(), row.volume());
            trades.push(bar);
        }

        Ok(trades)
    }
}
