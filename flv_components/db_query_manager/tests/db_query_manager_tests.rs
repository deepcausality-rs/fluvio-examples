use common::prelude::DBConfig;
use db_query_manager::QueryDBManager;

fn get_local_db_config() -> DBConfig {
    DBConfig::new(9009, "0.0.0.0".into())
}

#[test]
fn test_new() {
    let db_config = get_local_db_config();
    let manager = QueryDBManager::new(db_config);
    assert!(!manager.is_close());
    manager.close();
}

#[test]
fn test_close() {
    let db_config = get_local_db_config();
    let manager = QueryDBManager::new(db_config);
    manager.close();
}

#[test]
fn test_is_close() {
    let db_config = get_local_db_config();
    let manager = QueryDBManager::new(db_config);
    assert!(!manager.is_close());

    manager.close();
}

#[test]
fn test_get_all_symbol_ids() {
    let db_config = get_local_db_config();
    let mut manager = QueryDBManager::new(db_config);
    assert!(!manager.is_close());

    // symbol table
    let symbol_table = "kraken_symbols";

    // Call method under test
    let result = manager.get_all_symbols_with_ids(symbol_table);

    // Verify result
    assert!(result.is_ok());

    let symbols = result.unwrap();

    let expected_len = 695;
    let actual_len = symbols.len();
    assert_eq!(expected_len, actual_len);

    let expected_symbol_id = 1;
    let (actual_symbol_id, actual_symbol) = symbols.get(0).unwrap();

    assert_eq!(expected_symbol_id, *actual_symbol_id);

    let expected_symbol = &"apeusdt".to_string();
    assert_eq!(expected_symbol, actual_symbol);

    manager.close();
}
