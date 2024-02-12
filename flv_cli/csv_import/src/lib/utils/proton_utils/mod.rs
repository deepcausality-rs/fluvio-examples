use common::prelude::TradeBar;
use proton_client::ProtonClient;

pub fn insert_trade_bars(
    client: &ProtonClient,
    trade_bars: &Vec<TradeBar>,
    table_name: &str,
    symbol: &str,
    symbol_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn insert_meta_data(
    client: &ProtonClient,
    symbol: &str,
    symbol_id: i64,
    number_of_rows: i64,
    table_name: &str,
    meta_data_table: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
