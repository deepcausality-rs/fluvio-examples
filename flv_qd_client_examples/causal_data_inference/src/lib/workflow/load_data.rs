use crate::prelude::SampledDataBars;
use common::prelude::{ExchangeID, TimeResolution};
use db_query_manager::QueryDBManager;
use std::error::Error;
use config_manager::ConfigManager;
use symbol_manager::SymbolManager;

const FN_NAME: &'static str = "workflow/load_data";


/// Loads OHLCV bar data from the database at yearly and monthly resolutions.
///
/// Uses the provided [`QueryDBManager`] to query OHLCV bars for the specified
/// symbol id and table at yearly and monthly [`TimeResolution`]s.
///
/// The returned yearly and monthly bars are stored in a [`SampledDataBars`]
/// object.
///
/// # Arguments
///
/// * `db_manager` - [`QueryDBManager`] for executing database queries.
/// * `symbol_id` - ID of symbol to load data for.
/// * `symbol_table` - Table containing symbol data.
///
/// # Returns
///
/// Result with [`SampledDataBars`] containing the loaded data or error.
///
pub async fn load_data(
    cfg_manager: &ConfigManager,
    symbol_id: u16,
    exchange_id: ExchangeID,
) -> Result<SampledDataBars, Box<dyn Error>> {
    let mut bars = SampledDataBars::new();

    let default_exchange = cfg_manager.default_exchange();
    let exchanges = cfg_manager.exchanges_id_names().to_owned();
    let exchange_symbol_table = cfg_manager
        .get_symbol_table(default_exchange)
        .expect("[load_data]: Failed to get symbol table for default exchange.");

    println!("{FN_NAME}: Creating a new QueryDBManager.");
    let db_config = cfg_manager.db_config();
    let mut db_query_manager = QueryDBManager::new(db_config.clone())
        .await
        .expect("[load_data]: Failed to create QueryDBManager instance.");

    println!("{FN_NAME}: Get all symbols for the default exchange.");
    let symbols = db_query_manager
        .get_all_symbols_with_ids(&exchange_symbol_table)
        .await
        .expect("[load_data]: Failed to get all symbols for SymbolManager.");

    println!("{FN_NAME}: Creating a new SymbolManager.");
    let mut symbol_manager = SymbolManager::new(symbols, exchanges)
        .expect("[load_data]: Failed to create SymbolManager instance.");

    let symbol_table = symbol_manager
        .get_symbol_table_name(exchange_id as u16, symbol_id)
        .expect("[load_data]: Failed to get symbol table name");

    // Resample to yearly bars
    let time_resolution = &TimeResolution::OneYear;
    let result = db_query_manager
        .get_all_ohlcv_bars(symbol_id, &symbol_table, time_resolution)
        .await;

    // replace with error handling
    assert!(result.is_ok());

    let year_bars = result.unwrap();

    // Set year bars
    bars.set_year_bars(year_bars);

    // Resample to monthly bars
    let time_resolution = &TimeResolution::OneMonth;
    let result = db_query_manager
        .get_all_ohlcv_bars(symbol_id, &symbol_table, time_resolution)
        .await;

    // replace with error handling
    assert!(result.is_ok());

    let months_bars = result.unwrap();

    // Set month bars
    bars.set_month_bars(months_bars);

    Ok(bars)
}
