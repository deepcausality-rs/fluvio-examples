use common::prelude::SampledDataBars;
use common::prelude::{ExchangeID, TimeResolution};
use config_manager::ConfigManager;
use db_query_manager::QueryDBManager;
use std::error::Error;
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
/// * `cfg_manager` - [`ConfigManager`] instance for loading configuration.
/// * `symbol_id` - ID of symbol to load data for.
/// * `exchange_id` - [`ExchangeID`] of exchange symbol belongs to.
///
/// # Returns
///
/// Result with [`SampledDataBars`] containing the loaded data or error.
///
/// # Example
///
/// ```rust
/// use common::prelude::{ExchangeID, ServiceID};
/// use config_manager::ConfigManager;
///
/// async fn test(){
///
/// use client_utils::data_utils::load_data;
/// let cfg_manager = ConfigManager::new(ServiceID::Default);
/// let symbol = "ethaed";
/// let exchange_id = ExchangeID::Kraken;
///
/// let result = load_data(&cfg_manager, symbol, exchange_id).await;
///
/// assert!(result.is_ok());
/// }
/// ```
///
pub async fn load_data(
    cfg_manager: &ConfigManager,
    symbol: &str,
    exchange_id: ExchangeID,
) -> Result<SampledDataBars, Box<dyn Error>> {
    //
    let mut bars = SampledDataBars::new();

    // println!("{FN_NAME}: Loading configuration for QueryDBManager.");
    let default_exchange = cfg_manager.default_exchange();
    let exchanges = cfg_manager.exchanges_id_names().to_owned();
    let exchange_symbol_table = cfg_manager
        .get_symbol_table(default_exchange)
        .expect("[load_data]: Failed to get symbol table for default exchange.");

    // println!("{FN_NAME}: Creating a new QueryDBManager.");
    let db_config = cfg_manager.db_config();
    let mut db_query_manager = QueryDBManager::new(db_config.clone())
        .await
        .expect("[load_data]: Failed to create QueryDBManager instance.");

    // println!("{FN_NAME}: Get all symbols for the default exchange.");
    let symbols = db_query_manager
        .get_all_symbols_with_ids(&exchange_symbol_table)
        .await
        .expect("[load_data]: Failed to get all symbols for SymbolManager.");

    // println!("{FN_NAME}: Creating a new SymbolManager.");
    let mut symbol_manager = SymbolManager::new(symbols, exchanges)
        .expect("[load_data]: Failed to create SymbolManager instance.");

    // println!("{FN_NAME}: Get symbol id for symbol {}.", symbol);
    let symbol_id = symbol_manager
        .get_symbol_id(symbol)
        .expect("[load_data]: Failed to get symbol.");

    // println!("{FN_NAME}: Get symbol table for the default exchange.");
    let symbol_table = symbol_manager
        .get_symbol_table_name(exchange_id as u16, symbol_id)
        .expect("[load_data]: Failed to get symbol table name");

    // println!("{FN_NAME}: Get yearly bars for symbol {}.", symbol_id);
    let time_resolution = &TimeResolution::OneYear;
    let result = db_query_manager
        .get_all_ohlcv_bars(symbol_id, &symbol_table, time_resolution)
        .await;

    // println!("{FN_NAME}: Check for query error.");
    let year_bars = match result {
        Ok(bars) => bars,
        Err(err) => {
            println!(
                "{FN_NAME}: Failed to get yearly bars for symbol {}: {}",
                symbol_id, err
            );
            return Err(Box::new(err));
        }
    };

    // println!("{FN_NAME}: Set year bars.");
    bars.set_year_bars(year_bars);

    // println!("{FN_NAME}: Get monthly bars.");
    let time_resolution = &TimeResolution::OneMonth;
    let result = db_query_manager
        .get_all_ohlcv_bars(symbol_id, &symbol_table, time_resolution)
        .await;

    // println!("{FN_NAME}: Check for query error.");
    let months_bars = match result {
        Ok(bars) => bars,
        Err(err) => {
            println!(
                "{FN_NAME}: Failed to get monthly bars for symbol {}: {}",
                symbol_id, err
            );
            return Err(Box::new(err));
        }
    };

    // println!("{FN_NAME}: Set month bars.");
    bars.set_month_bars(months_bars);

    // Close DB connection
    db_query_manager.close().await;

    Ok(bars)
}
