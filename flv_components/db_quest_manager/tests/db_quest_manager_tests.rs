use common::prelude::{csv_utils, DBConfig};
use db_quest_manager::QuestDBManager;

const PATH: &str =
    "/Users/marvin/RustroverProjects/fluvio-examples/data/Kraken_Trading_History/ADAUSD.csv";

fn get_local_db_config() -> DBConfig {
    DBConfig::new(9009, "0.0.0.0".into())
}

#[test]
fn test_insert_trade_bars() {
    let db_config = get_local_db_config();

    let mut db_manager = QuestDBManager::new(db_config);

    let trade_bars = csv_utils::read_csv_file(PATH);

    assert!(trade_bars.is_ok());

    let trade_bars = trade_bars.unwrap();

    assert!(!trade_bars.is_empty());

    let table_name = "KRAKEN_ADA_USD".to_lowercase();
    let symbol = "ADA_USD".to_lowercase();
    let symbol_id = 1;
    let meta_data_table = "kraken_symbols";

    let result =
        db_manager.insert_trade_bars(trade_bars, &table_name, &symbol, symbol_id, meta_data_table);

    assert!(result.is_ok());
}
