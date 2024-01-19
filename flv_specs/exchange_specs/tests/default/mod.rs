

#[cfg(test)]
mod tests {
    use common::prelude::ExchangeID;
    use exchange_specs::prelude::{get_all_exchanges, get_all_exchanges_ids_names, get_exchange_symbol_tables};

    const KRK_SYMBOL_TABLE: &str = "kraken_symbols";

    #[test]
    fn test_get_all_exchanges() {
        let exchanges = get_all_exchanges();

        assert_eq!(exchanges.len(), 1);
        assert_eq!(exchanges[0], ExchangeID::Kraken);
    }

    #[test]
    fn test_get_all_exchanges_ids_names() {
        let id_names = get_all_exchanges_ids_names();

        assert_eq!(id_names.len(), 1);
        assert_eq!(id_names[0], (ExchangeID::Kraken as u16, "kraken".to_string()));
    }

    #[test]
    fn test_get_exchange_symbol_tables() {
        let tables = get_exchange_symbol_tables();

        assert_eq!(tables.len(), 1);
        assert_eq!(tables[&ExchangeID::Kraken], KRK_SYMBOL_TABLE);
    }
}
