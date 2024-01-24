use crate::prelude::SampledDataBars;
use common::prelude::TimeResolution;
use db_query_manager::QueryDBManager;
use std::error::Error;

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
    db_manager: &mut QueryDBManager,
    symbol_id: u16,
    symbol_table: &str,
) -> Result<SampledDataBars, Box<dyn Error>> {
    let mut bars = SampledDataBars::new();

    // Resample to yearly bars
    let time_resolution = &TimeResolution::OneYear;
    let result = db_manager
        .get_all_ohlcv_bars(symbol_id, symbol_table, time_resolution)
        .await;

    // Verify result
    assert!(result.is_ok());

    let year_bars = result.unwrap();

    // Set year bars
    bars.set_year_bars(year_bars);

    // Resample to monthly bars
    let time_resolution = &TimeResolution::OneMonth;
    let result = db_manager
        .get_all_ohlcv_bars(symbol_id, symbol_table, time_resolution)
        .await;

    // Verify result
    assert!(result.is_ok());

    let months_bars = result.unwrap();

    // Set month bars
    bars.set_month_bars(months_bars);

    Ok(bars)
}
