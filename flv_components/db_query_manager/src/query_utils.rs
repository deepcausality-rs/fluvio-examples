use crate::error::QueryError;
use crate::QueryDBManager;
use common::prelude::{TimeResolution, ValidationError};

impl QueryDBManager {
    /// Utils that executes a SQL query against the database.
    ///
    /// # Arguments
    ///
    /// * `query` - The SQL query string to execute
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `postgres::Row` representing the result rows if the query
    /// executed successfully. Returns a `postgres::Error` if there was an error executing the query.
    ///
    pub(crate) async fn query(
        &mut self,
        query: &str,
    ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
        let client = self
            .pool
            .get()
            .await
            .expect("[QueryDBManager/query]: Failed to get client from connection pool");

        client.query(query, &[]).await
    }

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
    pub(crate) fn build_get_symbol_id_query(&self, symbol_table: &str) -> String {
        format!("SELECT symbol_id, symbol FROM {};", symbol_table)
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
    pub(crate) fn build_get_trades_query(&self, trade_table: &str) -> String {
        format!("SELECT timestamp, price, volume FROM {};", trade_table)
    }

    pub(crate) fn build_get_ohlcv_bars_query(
        &self,
        trade_table: &str,
        time_resolution: &TimeResolution,
    ) -> String {
        format!(
            "SELECT
              timestamp datetime,
              first(price) open,
              max(price) high,
              min(price) low,
              last(price) close,
              sum(volume) volume,

            FROM {}
            SAMPLE BY {}
            ALIGN TO CALENDAR WITH OFFSET '00:00';",
            trade_table, time_resolution,
        )
    }

    /// Sanitizes the provided table name to prevent SQL injection attacks.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The table name to sanitize
    ///
    /// # Returns
    ///
    /// A `Result` containing the original table name if valid, or a `QueryError`
    /// if the name is invalid.
    ///
    /// # Errors
    ///
    /// - `QueryError::EmptyTableName` if `table_name` is empty
    /// - `QueryError::InvalidTableName` if `table_name` contains invalid characters
    /// - `QueryError::TableNameTooLong` if `table_name` is longer than 64 characters
    ///
    ///
    /// This checks `table_name` for:
    ///
    /// - Emptiness
    /// - Invalid characters
    /// - Length less than 64 characters
    ///
    /// If valid, it returns the original `table_name`.
    pub(crate) fn sanitize_table_name<'l>(
        &self,
        table_name: &'l str,
    ) -> Result<&'l str, QueryError> {
        // check for empty name
        if table_name.is_empty() {
            return Err(QueryError::EmptyTableName(ValidationError::new(format!(
                "Table: {}",
                table_name
            ))));
        }

        // check for invalid characters
        if table_name.chars().any(|c| !c.is_alphanumeric() && c != '_') {
            return Err(QueryError::InvalidTableName(ValidationError::new(format!(
                "Table: {}",
                table_name
            ))));
        }

        // check for length
        if table_name.len() > 64 {
            return Err(QueryError::TableNameTooLong(ValidationError::new(format!(
                "Table: {}",
                table_name
            ))));
        }

        Ok(table_name)
    }
}
