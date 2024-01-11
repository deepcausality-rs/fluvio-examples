mod getters;
mod lookup_exchange_name;
mod lookup_symbol;
mod lookup_symbol_table;

use common::prelude::InitError;
use lru::LruCache;
use std::collections::HashMap;
use std::num::NonZeroUsize;

pub struct SymbolManager {
    // Hashmaps to map between symbols and indices
    symbol_to_index: HashMap<String, u16>,
    index_to_symbol: HashMap<u16, String>,
    //  Exchange hashmaps to map between Exchange ID (u16) and Exchange Name (String)
    index_to_exchange: HashMap<u16, String>,

    // LRU Caches for faster symbol look-ups
    // https://docs.rs/lru/latest/lru/
    symbol_cache: LruCache<String, u16>,
    id_cache: LruCache<u16, String>,

    // Tracks number of symbols
    number_of_symbols: usize,
    // Tracks number of exchanges
    number_of_exchanges: usize,
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
    /// ```rust
    /// use common::prelude::DBConfig;
    /// use symbol_manager::SymbolManager;
    ///
    ///  let exchanges = vec![(1, "kraken".to_string()), (2, "bittrex".to_string())];
    ///  let symbols =  vec![(1, "apeusdt".to_string()), (2, "btxusdt".to_string())];
    ///
    ///  let symbol_manager = SymbolManager::new(symbols, exchanges)
    ///         .expect("Failed to create symbol manager");
    ///
    /// let nr_symbols = symbol_manager.number_of_symbols();
    /// println!("Symbol: {}", nr_symbols);
    /// ```
    ///
    /// # Noteworthy
    ///
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
    pub fn new(
        symbols: Vec<(u16, String)>,
        exchanges: Vec<(u16, String)>,
    ) -> Result<Self, InitError> {
        // Determine the capacity of the two hashmaps
        let symbols_capacity = symbols.len();
        let exchanges_capacity = exchanges.len();

        // Set the capacity of the LRU caches at either 10% of the hashmaps capacity
        // or 150 if the percentage calculation results in a None value.
        let n = symbols_capacity / 10;
        let cache_capacity = NonZeroUsize::new(n).unwrap_or_else(|| {
            NonZeroUsize::new(150).expect("Failed to determine LRU cache capacity")
        });

        // Create hashmaps
        let mut symbol_to_index = HashMap::with_capacity(symbols_capacity);
        let mut index_to_symbol = HashMap::with_capacity(symbols_capacity);
        let mut index_to_exchange = HashMap::with_capacity(exchanges_capacity);

        // Insert all symbols and matching ids into the symbol hashmaps
        for (id, symbol) in symbols {
            symbol_to_index.insert(symbol.clone(), id);
            index_to_symbol.insert(id, symbol.clone());
        }

        // Insert all exchange ids and names into the exchange hashmap
        for (id, name) in exchanges {
            index_to_exchange.insert(id, name);
        }

        Ok(SymbolManager {
            symbol_to_index,
            index_to_symbol,
            index_to_exchange,
            symbol_cache: LruCache::new(cache_capacity),
            id_cache: LruCache::new(cache_capacity),
            number_of_symbols: symbols_capacity,
            number_of_exchanges: exchanges_capacity,
        })
    }
}
