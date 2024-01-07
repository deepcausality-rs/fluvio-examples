use crate::SymbolManager;

/// Returns the number of symbols stored in this SymbolManager.
///
/// # Parameters
///
/// * `&self` - Reference to the SymbolManager instance.
///
/// # Returns
///
/// The number of symbols as a `usize`.
///
/// # Functionality
///
/// Simply returns the `number_of_symbols` field which tracks the total count of
/// symbols in this SymbolManager.
impl SymbolManager {
    pub fn number_of_symbols(&self) -> usize {
        self.number_of_symbols
    }
}
