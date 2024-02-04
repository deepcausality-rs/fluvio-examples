use common::prelude::{DBConfig, TimeResolution};
use db_query_manager::QueryDBManager;
use std::str::FromStr;
use futures::StreamExt;

fn get_local_db_config() -> DBConfig {
    DBConfig::new(9009, "0.0.0.0".to_string())
}

#[tokio::test]
async fn test_new_query_db_manager() {
    let db_config = get_local_db_config();
    let manager = QueryDBManager::new(db_config)
        .await
        .expect("Failed to create db connection");

    assert!(!manager.is_close().await);

    manager.close().await;
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

    let expected_len = 708;
    let actual_len = symbols.len();
    assert_eq!(expected_len, actual_len);

    let expected_symbol_id = 1;
    let (actual_symbol_id, actual_symbol) = symbols.first().unwrap();

    assert_eq!(expected_symbol_id, *actual_symbol_id);

    let expected_symbol = &"apeusdt".to_string();
    assert_eq!(expected_symbol, actual_symbol);

    // Close connection
    manager.close().await;
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
    let symbol_id = 284; // 284 = ethaed on Kraken

    // Call method under test
    let result = manager.get_all_trades(symbol_id, trade_table).await;

    // Verify result is ok
    assert!(result.is_ok());

    // Close connection
    manager.close().await;
}

#[tokio::test]
async fn test_stream_trades() {
    let db_config = get_local_db_config();
    let manager = QueryDBManager::new(db_config)
        .await
        .expect("Failed to create db connection");
    assert!(!manager.is_close().await);

    // trade table name
    // ethaed has only 43 records so this is a good and fast test
    let trade_table = "kraken_ethaed";
    let symbol_id = 284; // 284 = ethaed on Kraken

    // Call method under tes
    let mut stream = manager.stream_trades(symbol_id, trade_table).await;

    while let  Some(record) = stream.next().await {
        assert!(record.is_ok());
        let record = record.unwrap();
        println!("Got {:?}", record);
    }

    assert!(!manager.is_close().await);

    // Close connection
    manager.close().await;
}

#[tokio::test]
async fn test_get_all_ohlcv_bars() {
    let db_config = get_local_db_config();
    let mut manager = QueryDBManager::new(db_config)
        .await
        .expect("Failed to create db connection");
    assert!(!manager.is_close().await);

    // trade table name
    // ethaed has only 43 records so this is a good and fast test
    let trade_table = "kraken_ethaed";
    let time_resolution = &TimeResolution::FifteenMin;
    let symbol_id = 284; // 284 = ethaed on Kraken

    // Resample to 15 min bars
    let result = manager
        .get_all_ohlcv_bars(symbol_id, trade_table, time_resolution)
        .await;

    // Verify result
    assert!(result.is_ok());

    let bars = result.unwrap();
    let expected_len = 36;
    let actual_len = bars.len();
    assert_eq!(expected_len, actual_len);

    // Verify first bar
    let expected_open = rust_decimal::Decimal::from_str("6306.1").unwrap();
    let expected_high = rust_decimal::Decimal::from_str("6313.5").unwrap();
    let expected_low = rust_decimal::Decimal::from_str("6276.6").unwrap();
    let expected_close = rust_decimal::Decimal::from_str("6276.6").unwrap();
    let expected_volume = rust_decimal::Decimal::from_str("0.01764591").unwrap();

    let first_bar = bars.first().unwrap();

    assert_eq!(expected_open, first_bar.open());
    assert_eq!(expected_high, first_bar.high());
    assert_eq!(expected_low, first_bar.low());
    assert_eq!(expected_close, first_bar.close());
    assert_eq!(expected_volume, first_bar.volume());

    // Close connection
    manager.close().await;
}
