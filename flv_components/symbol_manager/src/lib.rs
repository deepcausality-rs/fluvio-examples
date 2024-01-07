mod lookup;

use db_query_manager::QueryDBManager;
use lru::LruCache;
use std::collections::HashMap;
use std::num::NonZeroUsize;

pub struct SymbolManager {
    // First hashmap that maps from symbol to index
    symbol_to_index: HashMap<String, u16>,
    // Second hashmap that maps from index to symbol
    index_to_symbol: HashMap<u16, String>,

    // LRU caches for symbol and id look ups
    symbol_cache: LruCache<String, u16>,
    id_cache: LruCache<u16, String>,

    number_of_symbols: usize,
}

impl SymbolManager {
    pub fn new(query_db_manager: &mut QueryDBManager) -> SymbolManager {
        //
        let symbols = query_db_manager
            .get_all_symbols_with_ids("kraken_symbols")
            .expect("Failed to query all symbols from symbols table");

        let capacity = symbols.len();

        let mut symbol_to_index = HashMap::with_capacity(capacity);
        let mut index_to_symbol = HashMap::with_capacity(capacity);

        for (id, symbol) in symbols {
            symbol_to_index.insert(symbol.clone(), id);
            index_to_symbol.insert(id, symbol.clone());
        }

        // Implicitly drop QueryDBManager so that SymbolManager
        // is thread-safe to use via clone.
        SymbolManager {
            symbol_to_index,
            index_to_symbol,
            symbol_cache: LruCache::new(NonZeroUsize::new(50).unwrap()),
            id_cache: LruCache::new(NonZeroUsize::new(50).unwrap()),
            number_of_symbols: capacity,
        }
    }
}

impl SymbolManager {
    pub fn number_of_symbols(&self) -> usize {
        self.number_of_symbols
    }
}
