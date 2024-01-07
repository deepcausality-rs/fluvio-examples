use crate::SymbolManager;
use common::prelude::LookupError;

impl SymbolManager {
    /// Retrieves the symbol string for the given symbol ID.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - The numeric ID of the symbol to look up
    ///
    /// # Returns
    ///
    /// Returns a Result with the symbol string if found, or a LookupError if not found.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    /// use symbol_manager::SymbolManager;
    ///
    ///  let db_config = DBConfig::new(9009, "0.0.0.0".into());
    ///  let mut symbol_manager = SymbolManager::new(db_config)
    ///         .expect("Failed to create symbol manager");
    ///
    /// let symbol = symbol_manager.get_symbol(1).expect("Failed to get symbol");
    /// println!("Symbol: {}", symbol);
    /// ```
    ///
    /// # Notes
    ///
    /// First checks the id_cache before doing a lookup in index_to_symbol.
    /// Puts any fetched symbols into the cache before returning.
    pub fn get_symbol(&mut self, symbol_id: u16) -> Result<String, LookupError> {
        if let Some(symbol) = self.id_cache.get(&symbol_id) {
            return Ok(symbol.clone());
        }

        let symbol = match self.index_to_symbol.get(&symbol_id) {
            Some(symbol) => symbol,
            None => {
                return Err(LookupError::new(format!(
                    "Symbol not found for ID: {}",
                    symbol_id
                )))
            }
        };

        self.id_cache.put(symbol_id, symbol.to_owned());

        Ok(symbol.to_owned())
    }

    /// Retrieves the ID for the given symbol string.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol string to look up
    ///
    /// # Returns
    ///
    /// Returns a Result with the ID if found, or a LookupError if not found.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    /// use symbol_manager::SymbolManager;
    ///
    ///  let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    ///  let mut symbol_manager = SymbolManager::new(db_config)
    ///         .expect("Failed to create symbol manager");
    ///
    /// let id = symbol_manager.get_symbol_id("apeusdt").expect("Failed to get ID");
    /// println!("ID: {}", id);
    /// ```
    ///
    /// # Notes
    ///
    /// First checks the symbol_cache before doing a lookup in symbol_to_index.
    /// Puts any fetched IDs into the cache before returning.
    pub fn get_symbol_id(&mut self, symbol: &str) -> Result<u16, LookupError> {
        if let Some(&id) = self.symbol_cache.get(symbol) {
            return Ok(id);
        }

        let id = match self.symbol_to_index.get(symbol) {
            Some(id) => id,
            None => {
                return Err(LookupError::new(format!(
                    "ID not found for Symbol: {}",
                    symbol
                )))
            }
        };

        self.symbol_cache.put(symbol.to_owned(), *id);

        Ok(*id)
    }

    /// Retrieves all available symbol strings.
    ///
    /// # Returns
    ///
    /// Returns a Result with a vector of String symbols if available,
    /// or a LookupError if no symbols are found.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    /// use symbol_manager::SymbolManager;
    ///
    ///  let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    ///  let mut symbol_manager = SymbolManager::new(db_config)
    ///         .expect("Failed to create symbol manager");
    ///
    /// let symbols = symbol_manager.get_all_symbols();
    /// println!("Symbols: {:?}", symbols);
    /// ```
    ///
    /// # Notes
    ///
    /// Checks if symbol_to_index map is empty first.
    /// If not empty, collects all keys into a vector to return.
    pub fn get_all_symbols(&self) -> Result<Vec<String>, LookupError> {
        if self.symbol_to_index.is_empty() {
            return Err(LookupError::new("No symbols found".to_string()));
        }

        let symbols: Vec<String> = self.symbol_to_index.keys().cloned().collect();
        Ok(symbols)
    }

    /// Retrieves all symbol IDs available.
    ///
    /// # Returns
    ///
    /// Returns a Result with a vector of u16 symbol IDs if available,
    /// or a LookupError if no symbol IDs are found.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    /// use symbol_manager::SymbolManager;
    ///
    ///  let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    ///  let mut symbol_manager = SymbolManager::new(db_config)
    ///         .expect("Failed to create symbol manager");
    ///
    ///  let result = symbol_manager.get_all_symbol_ids();
    ///  assert!(result.is_ok());
    ///
    ///  let ids = result.unwrap();
    ///  assert!(ids.contains(&23));
    ///
    ///  println!("Symbol IDs: {:?}", ids);
    /// ```
    ///
    /// # Notes
    ///
    /// Checks if index_to_symbol map is empty first.
    /// If not empty, collects all keys into a vector to return.
    pub fn get_all_symbol_ids(&self) -> Result<Vec<u16>, LookupError> {
        if self.index_to_symbol.is_empty() {
            return Err(LookupError::new("No symbol IDs found".to_string()));
        }

        let ids: Vec<u16> = self.index_to_symbol.keys().cloned().collect();
        Ok(ids)
    }
}
