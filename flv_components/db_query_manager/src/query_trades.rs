use crate::error::QueryError;
use crate::QueryDBManager;
use common::prelude::TradeBar;

impl QueryDBManager {
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
            // Convert the row to a trade bar
            let trade = match self.build_trade_bar_from_row(row) {
                Ok(trade) => trade,
                Err(e) => return Err(e),
            };

            trades.push(trade);
        }

        Ok(trades)
    }
}
