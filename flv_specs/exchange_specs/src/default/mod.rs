use common::prelude::ExchangeID;
use std::collections::HashMap;

const KRK_SYMBOL_TABLE: &str = "kraken_symbols";

/// Get all supported exchanges.
///
/// # Returns
///
/// A vector containing all supported ExchangeID variants.
/// Currently only returns Kraken.
pub fn get_all_exchanges() -> Vec<ExchangeID> {
    vec![ExchangeID::Kraken]
}
/// Get a vector of exchange ID and name pairs.
///
/// # Returns
///
/// A vector of tuples containing the u16 ID and name string
/// for each supported exchange. Currently only returns Kraken.
pub fn get_all_exchanges_ids_names() -> Vec<(u16, String)> {
    vec![(ExchangeID::Kraken as u16, "kraken".to_string())]
}

/// Get a HashMap of symbol tables for supported exchanges.
///
/// The key is the ExchangeID and the value is the symbol table name.
///
/// # Returns
///
/// A HashMap mapping ExchangeID to symbol table name string.
/// Currently only contains mapping for Kraken.
pub fn get_exchange_symbol_tables() -> HashMap<ExchangeID, String> {
    let mut tables = HashMap::new();
    tables.insert(ExchangeID::Kraken, KRK_SYMBOL_TABLE.to_string());

    tables
}
