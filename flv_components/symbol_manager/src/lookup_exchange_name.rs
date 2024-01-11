use crate::SymbolManager;
use common::prelude::LookupError;

impl SymbolManager {
    /// Looks up the name of an exchange by its ID.
    ///
    /// # Parameters
    ///
    /// * `exchange_id` - The ID of the exchange to look up.
    ///
    /// # Returns
    ///
    /// Returns the name of the exchange as a `Result<String, LookupError>`.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use symbol_manager::SymbolManager;
    ///
    ///  let exchanges = vec![(1, "kraken".to_string()), (2, "bittrex".to_string())];
    ///  let symbols =  vec![(1, "apeusdt".to_string()), (2, "btxusdt".to_string())];
    ///
    ///  let mut symbol_manager = SymbolManager::new(symbols, exchanges)
    ///         .expect("Failed to create symbol manager");
    ///
    /// let exchange_id = 1;
    /// let exchange_name = symbol_manager.get_exchange_name(exchange_id)
    ///                                 .expect("Failed to get exchange name");
    ///
    /// println!("exchange: {}", exchange_name);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `LookupError` if no exchange with the given ID exists.
    ///
    /// # Functionality
    ///
    /// Looks up the `exchange_id` key in the `index_to_exchange` map.
    /// If found, returns the exchange name.
    /// If not found, returns a `LookupError`.
    pub fn get_exchange_name(&mut self, exchange_id: u16) -> Result<String, LookupError> {
        let exchange_name = match self.index_to_exchange.get(&exchange_id) {
            Some(symbol) => symbol,
            None => {
                return Err(LookupError::new(format!(
                    "Exchange not found for ID: {}",
                    exchange_id
                )));
            }
        };

        Ok(exchange_name.to_owned())
    }
}
