use symbol_manager::SymbolManager;

fn get_test_symbols() -> Vec<(u16, String)> {
    vec![
        (1, "apeusdt".to_string()),
        (2, "btxusdt".to_string()),
        (3, "xrpusd".to_string()),
    ]
}

#[test]
fn test_new() {
    let db_config = get_test_symbols();

    let symbol_manager = SymbolManager::new(db_config).expect("Failed to create symbol manager");

    assert_eq!(symbol_manager.number_of_symbols(), 3);
}

#[test]
fn test_get_symbol() {
    let db_config = get_test_symbols();
    let mut symbol_manager =
        SymbolManager::new(db_config).expect("Failed to create symbol manager");

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
    let db_config = get_test_symbols();
    let mut symbol_manager =
        SymbolManager::new(db_config).expect("Failed to create symbol manager");

    // Cache miss
    let id = symbol_manager.get_symbol_id("apeusdt").unwrap();
    assert_eq!(id, 1);

    // Cache hit
    let id = symbol_manager.get_symbol_id("apeusdt").unwrap();
    assert_eq!(id, 1);

    // ID not found for symbol
    let result = symbol_manager.get_symbol_id("lalalcoin");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "LookupError: ID not found for Symbol: lalalcoin"
    );
}

#[test]
fn test_get_all_symbols() {
    let db_config = get_test_symbols();
    let symbol_manager = SymbolManager::new(db_config).expect("Failed to create symbol manager");

    let results = symbol_manager.get_all_symbols();
    assert!(results.is_ok());

    let symbols = results.unwrap();

    assert_eq!(symbols.len(), 3);
    assert!(symbols.contains(&"xrpusd".to_string()));
}

#[test]
fn test_get_all_get_all_symbol_ids() {
    let db_config = get_test_symbols();
    let symbol_manager = SymbolManager::new(db_config).expect("Failed to create symbol manager");

    let result = symbol_manager.get_all_symbol_ids();
    assert!(result.is_ok());

    let ids = result.unwrap();

    assert_eq!(ids.len(), 3);
    assert!(ids.contains(&3));
}
