use symbol_manager::SymbolManager;

fn get_test_exchanges() -> Vec<(u16, String)> {
    vec![(1, "kraken".to_string())]
}

fn get_test_symbols() -> Vec<(u16, String)> {
    vec![
        (1, "apeusdt".to_string()),
        (2, "btxusdt".to_string()),
        (3, "xrpusd".to_string()),
    ]
}

#[test]
fn test_new() {
    let exchanges = get_test_exchanges();
    let symbols = get_test_symbols();

    let symbol_manager =
        SymbolManager::new(symbols, exchanges).expect("Failed to create symbol manager");

    assert_eq!(symbol_manager.number_of_symbols(), 3);
}

#[test]
fn test_get_symbol() {
    let exchanges = get_test_exchanges();
    let symbols = get_test_symbols();

    let mut symbol_manager =
        SymbolManager::new(symbols, exchanges).expect("Failed to create symbol manager");

    // Cache miss
    let symbol = symbol_manager.get_symbol(1).unwrap();
    assert_eq!(symbol, "apeusdt");

    // Cache hit
    let symbol = symbol_manager.get_symbol(1).unwrap();
    assert_eq!(symbol, "apeusdt");

    // Symbol not found
    let result = symbol_manager.get_symbol(9999);
    assert!(result.is_err());
}

#[test]
fn test_get_symbol_id() {
    let exchanges = get_test_exchanges();
    let symbols = get_test_symbols();
    let mut symbol_manager =
        SymbolManager::new(symbols, exchanges).expect("Failed to create symbol manager");

    // Cache miss
    let id = symbol_manager.get_symbol_id("apeusdt").unwrap();
    assert_eq!(id, 1);

    // Cache hit
    let id = symbol_manager.get_symbol_id("apeusdt").unwrap();
    assert_eq!(id, 1);

    // ID not found for symbol
    let result = symbol_manager.get_symbol_id("lalacoin");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "LookupError: ID not found for Symbol: lalacoin"
    );
}

#[test]
fn test_get_all_symbols() {
    let exchanges = get_test_exchanges();
    let symbols = get_test_symbols();
    let symbol_manager =
        SymbolManager::new(symbols, exchanges).expect("Failed to create symbol manager");

    let results = symbol_manager.get_all_symbols();
    assert!(results.is_ok());

    let symbols = results.unwrap();

    assert_eq!(symbols.len(), 3);
    assert!(symbols.contains(&"xrpusd".to_string()));
}

#[test]
fn test_get_all_symbol_ids() {
    let exchanges = get_test_exchanges();
    let symbols = get_test_symbols();
    let symbol_manager =
        SymbolManager::new(symbols, exchanges).expect("Failed to create symbol manager");

    let result = symbol_manager.get_all_symbol_ids();
    assert!(result.is_ok());

    let ids = result.unwrap();

    assert_eq!(ids.len(), 3);
    assert!(ids.contains(&3));
}

#[test]
fn test_get_exchange_name() {
    let exchanges = get_test_exchanges();
    let symbols = get_test_symbols();
    let mut symbol_manager =
        SymbolManager::new(symbols, exchanges).expect("Failed to create symbol manager");

    let exchange_id = 1;
    let exchange_name = symbol_manager
        .get_exchange_name(exchange_id)
        .expect("Failed to get exchange name");

    assert_eq!(exchange_name, "kraken");
}

#[test]
fn test_get_symbol_table_name() {
    let exchanges = get_test_exchanges();
    let symbols = get_test_symbols();
    let mut symbol_manager =
        SymbolManager::new(symbols, exchanges).expect("Failed to create symbol manager");

    let exchange_id = 1;
    let symbol_id = 2;
    let symbol_table_name = symbol_manager
        .get_symbol_table_name(exchange_id, symbol_id)
        .expect("Failed to get symbol table name");

    assert_eq!(symbol_table_name, "kraken_btxusdt");
}
