use common::prelude::DBConfig;
use db_query_manager::QueryDBManager;

fn get_local_db_config() -> DBConfig {
    DBConfig::new(9009, "0.0.0.0".into())
}

#[tokio::test]
async fn test_new_query_db_manager() {
    let db_config = get_local_db_config();
    let manager = QueryDBManager::new(db_config)
        .await
        .expect("Failed to create db connection");

    assert!(!manager.is_close().await);
}

#[tokio::test]
async fn test_get_all_symbol_ids() {
    let db_config = get_local_db_config();
    let mut manager = QueryDBManager::new(db_config)
        .await
        .expect("Failed to create db connection");
    assert!(!manager.is_close().await);

    // symbol table
    let symbol_table = "kraken_symbols";

    // Call method under test
    let result = manager.get_all_symbols_with_ids(symbol_table).await;
    // Verify result
    assert!(result.is_ok());

    let symbols = result.unwrap();

    let expected_len = 695;
    let actual_len = symbols.len();
    assert_eq!(expected_len, actual_len);

    let expected_symbol_id = 1;
    let (actual_symbol_id, actual_symbol) = symbols.first().unwrap();

    assert_eq!(expected_symbol_id, *actual_symbol_id);

    let expected_symbol = &"apeusdt".to_string();
    assert_eq!(expected_symbol, actual_symbol);
}

#[tokio::test]
async fn test_get_all_trades() {
    let db_config = get_local_db_config();
    let mut manager = QueryDBManager::new(db_config)
        .await
        .expect("Failed to create db connection");
    assert!(!manager.is_close().await);

    // trade table name
    // ethaed has only 43 records so this is a good and fast test
    let trade_table = "kraken_ethaed";

    // Call method under test
    let result = manager.get_all_trades(trade_table).await;

    // Verify result
    assert!(result.is_ok());

    let trades = result.unwrap();

    let expected_len = 43;
    let actual_len = trades.len();
    assert_eq!(expected_len, actual_len);
}
