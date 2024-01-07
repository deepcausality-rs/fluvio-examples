mod lookup;
mod utils;

use common::prelude::{DBConfig, InitError};
use db_query_manager::QueryDBManager;
use lru::LruCache;
use std::collections::HashMap;
use std::num::NonZeroUsize;

pub struct SymbolManager {
    // Hashmaps to map between symbols and indices
    symbol_to_index: HashMap<String, u16>,
    index_to_symbol: HashMap<u16, String>,

    // LRU Caches for faster look-ups
    symbol_cache: LruCache<String, u16>,
    id_cache: LruCache<u16, String>,

    // Tracks number of symbols
    number_of_symbols: usize,
}

impl SymbolManager {
    /// Creates a new SymbolManager instance.
    ///
    /// # Parameters
    ///
    /// * `db_config` - The configuration to use for connecting to the symbol database.
    ///
    /// # Returns
    ///
    /// A Result containing the new SymbolManager instance or an InitError.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    /// use symbol_manager::SymbolManager;
    ///
    ///  let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    ///  let symbol_manager = SymbolManager::new(db_config)
    ///         .expect("Failed to create symbol manager");
    ///
    /// let nr_symbols = symbol_manager.number_of_symbols();
    /// println!("Symbol: {}", nr_symbols);
    /// ```
    ///
    /// # Noteworthy
    ///
    /// - Creates a temporary QueryDBManager instance from the provided config.
    ///
    /// - Queries for all symbols and ids using the QueryDBManager instance.
    ///
    /// - Determines hashmap capacities based on number of symbols.
    ///
    /// - Initializes the symbol_to_index and index_to_symbol hashmaps.
    ///
    /// - Inserts all symbols and ids into the hashmaps.
    ///
    /// - Closes the database connection and drops the QueryDBManager.
    ///
    /// - Creates and returns a SymbolManager instance containing the initialized
    ///   hashmaps and caches.
    ///
    /// - Propagates any errors via the returned Result.
    pub fn new(db_config: DBConfig) -> Result<Self, InitError> {
        // create a temporary mutable query_db_manager
        let mut query_db_manager = QueryDBManager::new(db_config);

        // query all symbols from the symbols table
        let symbols = match query_db_manager.get_all_symbols_with_ids("kraken_symbols") {
            Ok(symbols) => symbols,
            Err(e) => return Err(InitError::new(e.to_string())),
        };

        // Determine the capacity of the hashmaps
        let capacity = symbols.len();

        // Set the capacity of the LRU caches at either 10% of the hashmaps capacity
        // or 150 if the percentage calculation results in a None value.
        let n = capacity / 10;
        let cache_capacity = NonZeroUsize::new(n).unwrap_or_else(|| {
            NonZeroUsize::new(150).expect("Failed to determine LRU cache capacity")
        });

        // Create the hashmaps
        let mut symbol_to_index = HashMap::with_capacity(capacity);
        let mut index_to_symbol = HashMap::with_capacity(capacity);

        // Insert all symbols and matching ids into the hashmaps
        for (id, symbol) in symbols {
            symbol_to_index.insert(symbol.clone(), id);
            index_to_symbol.insert(id, symbol.clone());
        }

        // Close the connection to the database
        match query_db_manager.close() {
            Ok(_) => {}
            Err(e) => {
                return Err(InitError::new(e.to_string()));
            }
        }

        // Implicitly drop QueryDBManager

        // SymbolManager is thread-safe when using one clone per thread
        // or when wrapped in Arc/Mutex.
        Ok(SymbolManager {
            symbol_to_index,
            index_to_symbol,
            symbol_cache: LruCache::new(cache_capacity),
            id_cache: LruCache::new(cache_capacity),
            number_of_symbols: capacity,
        })
    }
}
