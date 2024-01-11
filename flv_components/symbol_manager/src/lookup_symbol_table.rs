use crate::SymbolManager;
use common::prelude::LookupError;

impl SymbolManager {
    /// Retrieves the database table name for the given exchange ID and symbol ID.
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The ID of the exchange to look up
    /// * `symbol_id` - The ID of the symbol to look up
    ///
    /// # Returns
    ///
    /// Returns a `String` containing the symbol table name in the format
    /// `{exchange_name}_{symbol}`, or a `LookupError` if the exchange or symbol
    /// could not be found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    /// use symbol_manager::SymbolManager;
    ///
    ///  let exchanges = vec![(1, "kraken".to_string()), (2, "bittrex".to_string())];
    ///  let symbols =  vec![(1, "apeusdt".to_string()), (2, "btxusdt".to_string())];
    ///
    ///  let mut  symbol_manager = SymbolManager::new(symbols, exchanges)
    ///         .expect("Failed to create symbol manager");
    ///
    /// let exchange_id = 1;
    /// let symbol_id = 2;
    /// let symbol_table_name = symbol_manager
    /// .get_symbol_table_name(exchange_id, symbol_id)
    ///         .expect("Failed to get symbol table name");
    ///
    /// assert_eq!(symbol_table_name, "kraken_btxusdt");
    ///
    /// ```
    ///
    /// # Notes
    ///
    /// The symbol table name is generated by looking up the exchange name and symbol
    /// based on the provided IDs, and concatenating them with an underscore.
    pub fn get_symbol_table_name(
        &mut self,
        exchange_id: u16,
        symbol_id: u16,
    ) -> Result<String, LookupError> {
        let symbol = match self.get_symbol(symbol_id) {
            Ok(symbol) => symbol,
            Err(err) => return Err(err),
        };

        let exchange_name = match self.get_exchange_name(exchange_id) {
            Ok(exchange) => exchange,
            Err(err) => return Err(err),
        };

        let symbol_table_name = format!("{}_{}", exchange_name, symbol);

        Ok(symbol_table_name)
    }

    /// Retrieves the database table name for the symbols on the given exchange.
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The ID of the exchange
    ///
    /// # Returns
    ///
    /// Returns a `String` containing the symbol table name in the format
    /// `{exchange_name}_symbols`, or a `LookupError` if the exchange could not be found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    /// use symbol_manager::SymbolManager;
    ///
    ///  let exchanges = vec![(1, "kraken".to_string()), (2, "bittrex".to_string())];
    ///  let symbols =  vec![(1, "apeusdt".to_string()), (2, "btxusdt".to_string())];
    ///
    ///  let mut  symbol_manager = SymbolManager::new(symbols, exchanges)
    ///         .expect("Failed to create symbol manager");
    ///
    /// let exchange_id = 1;
    ///
    /// let symbol_table_name = symbol_manager.get_symbol_table(exchange_id)
    ///     .expect("Failed to get symbol table name");
    ///
    /// assert_eq!(symbol_table_name, "kraken_symbols");
    /// ```
    pub fn get_symbol_table(
        &mut self,
        exchange_id: u16,
    ) -> Result<String, LookupError> {
        //
        let exchange_name = match self.get_exchange_name(exchange_id) {
            Ok(exchange) => exchange,
            Err(err) => return Err(err),
        };

        let symbol_table_name = format!("{}_symbols", exchange_name);

        Ok(symbol_table_name)
    }
}
