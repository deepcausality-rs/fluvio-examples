use crate::QueryDBManager;
use std::fmt::Error;

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
    /// `std::fmt::Error` if there was an error formatting the result.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    ///
    ///  let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    ///  let mut query_db_manager = QueryDBManager::new(db_config);
    ///
    /// let symbols = query_db_manager.get_all_symbols_with_ids("kraken_symbols")
    ///             .expect("Failed to query all symbols from kraken_symbols table");
    /// ```
    pub fn get_all_symbols_with_ids(
        &mut self,
        symbol_table: &str,
    ) -> Result<Vec<(u16, String)>, Error> {
        let query = self.build_get_symbol_id_query(symbol_table);

        let rows = self
            .query(&query)
            .expect("Failed to query all symbols from kraken_symbols table");

        let mut result: Vec<(u16, String)> = Vec::with_capacity(rows.len());

        for row in rows {
            let symbol_id: i64 = row.get(0);
            let symbol: String = row.get(1);

            result.push((symbol_id as u16, symbol));
        }

        Ok(result)
    }
}
