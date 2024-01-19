use common::prelude::ExchangeID;
use common::prelude::ExchangeID::Kraken;
use std::collections::HashMap;

const KRK_SYMBOL_TABLE: &str = "kraken_symbols";

pub fn get_all_exchanges() -> Vec<ExchangeID> {
    vec![ExchangeID::Kraken]
}

pub fn get_all_exchanges_ids_names() -> Vec<(u16, String)> {
    vec![(Kraken as u16, "kraken".to_string())]
}

pub fn get_exchange_symbol_tables() -> HashMap<ExchangeID, String> {
    let mut tables = HashMap::new();
    tables.insert(ExchangeID::Kraken, KRK_SYMBOL_TABLE.to_string());

    tables
}
