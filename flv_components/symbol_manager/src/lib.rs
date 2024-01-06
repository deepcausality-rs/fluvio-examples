use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolManager<'l> {
    // First hashmap that maps from symbol to index
    symbol_to_index: HashMap<&'l str, u16>,
    // Second hashmap that maps from index to symbol
    index_to_symbol: HashMap<u16, &'l str>,
}

impl<'l> SymbolManager<'l> {
    pub fn new() -> SymbolManager<'l> {
        // TODO:
        // * Connect to the DB
        // * Load symbols from the DB
        // * Generate both hashmaps
        // * Construct the symbol manager

        // Implicitly drop the DB connection to make sure it's closed
        // so that SymbolManager is thread-safe to use via clone.
        SymbolManager {
            symbol_to_index: Default::default(),
            index_to_symbol: Default::default(),
        }
    }
}
