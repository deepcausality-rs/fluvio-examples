use crate::error::QueryError;
use crate::types::SymbolRow;
use crate::{QueryDBManager, FN_NAME};

impl QueryDBManager {
    /// Retrieves all symbols and their IDs from the given symbol table.
    ///
    /// # Arguments
    ///
    /// * `symbol_table` - The name of the symbol table to query.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `(u16, String)` tuples, where the `u16` is the
    /// symbol ID and the `String` is the symbol name, if successful. Returns a
    ///
    /// `QueryError`
    /// - `QueryError::QueryFailed` if the query to the DB failed.
    /// - `QueryError::EmptyTableName` if `table_name` is empty
    /// - `QueryError::InvalidTableName` if `table_name` contains invalid characters
    /// - `QueryError::TableNameTooLong` if `table_name` is longer than 64 characters
    ///
    /// See wrapped errors for more details.
    ///
    /// # Example
    ///
    /// ```rust
    /// use common::prelude::ClickHouseConfig;
    /// use db_query_manager::QueryDBManager;
    /// #[tokio::main]
    /// async fn main() {
    /// let db_config =  ClickHouseConfig::default();
    ///  let mut query_manager = QueryDBManager::new(db_config).await.expect("Failed to create db connection");
    ///
    ///  let symbols = query_manager.get_all_symbols_with_ids("kraken_symbols")
    ///             .await
    ///             .expect("Failed to query all symbols from symbols table");
    ///
    /// }
    /// ```
    ///
    /// Note:
    ///
    /// This function first sanitizes the provided table name to prevent SQL injection.
    /// It then builds and executes a query to retrieve all rows from that table.
    /// The result rows are parsed into a vector of `(u16, String)` tuples containing
    /// the symbol ID and name respectively. Any errors are handled and returned in
    /// the `QueryError` enum.
    pub async fn get_all_symbols_with_ids(
        &mut self,
        symbol_table: &str,
    ) -> Result<Vec<(u16, String)>, QueryError> {
        // Sanitize table name input to prevent SQL injection.
        let sanitized_name = match self.sanitize_table_name(symbol_table) {
            Ok(name) => name,
            Err(e) => return Err(e),
        };

        // Build the query
        let query = self.build_get_symbol_id_query(sanitized_name);

        // Execute query
        let result_rows = self
            .client
            .query_collect::<SymbolRow>(&query)
            .await
            .expect(format!("{} Failed to execute query: {}", FN_NAME, query).as_str());

        // Check for empty result
        if result_rows.is_empty() {
            return Ok(Vec::new());
        }

        // Build the vector of tuples holding the symbol ID and symbol name.
        let mut symbol_id_name_pairs: Vec<(u16, String)> = Vec::with_capacity(result_rows.len());

        // Iterate through the rows and add the symbol ID and symbol name to the vector.
        for row in result_rows {
            let symbol_id: i64 = row.symbol_id() as i64;
            let symbol: String = row.symbol();

            symbol_id_name_pairs.push((symbol_id as u16, symbol));
        }

        // Return the vector of tuples.
        Ok(symbol_id_name_pairs)
    }
}
