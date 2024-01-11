use crate::SymbolManager;

impl SymbolManager {
    /// Returns the number of symbols stored in this SymbolManager.
    ///
    /// # Returns
    ///
    /// The number of symbols as a `usize`.
    ///
    /// # Functionality
    ///
    /// Simply returns the `number_of_symbols` field which tracks the total count of
    /// symbols in this SymbolManager.
    pub fn number_of_symbols(&self) -> usize {
        self.number_of_symbols
    }

    /// Returns the number of exchanges stored in this SymbolManager.
    ///
    /// # Returns
    ///
    /// The number of exchanges as a `usize`.
    ///
    /// # Functionality
    ///
    /// Simply returns the `number_of_exchanges` field which tracks the total count of
    /// exchanges in this SymbolManager.
    pub fn number_of_exchanges(&self) -> usize {
        self.number_of_exchanges
    }
}
