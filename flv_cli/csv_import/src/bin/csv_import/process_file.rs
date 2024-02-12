use lib_csv_import::utils::{csv_utils, proton_utils};
use proton_client::ProtonClient;
use std::error::Error;
use std::path::PathBuf;

pub fn process(
    client: &ProtonClient,
    file_path: &PathBuf,
    imported_files: i64,
    meta_data_table: &str,
) -> Result<(), Box<dyn Error>> {
    // get file name without extension
    let file = file_path
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert file name to str")
        .replace(".csv", "");

    // get file path
    let path = file_path
        .to_str()
        .expect("Failed to convert file path to str");

    // read CSV into TradeBars
    let trade_bars = match csv_utils::read_csv_file(path) {
        Ok(bars) => bars,
        Err(e) => return Err(e),
    };

    // skip empty data records
    if trade_bars.is_empty() {
        return Ok(());
    }

    // Construct some fields
    let number_of_rows = trade_bars.len() as i64;
    let table_name = format!("KRAKEN_{}", file).to_lowercase();
    let symbol = file.to_lowercase();
    let symbol_id = imported_files;

    // insert trade bars into Proton stream table
    match proton_utils::insert_trade_bars(&client, &trade_bars, &table_name, &symbol, symbol_id) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    // insert meta data into Proton meta data table
    match proton_utils::insert_meta_data(
        &client,
        &symbol,
        symbol_id,
        number_of_rows,
        &table_name,
        meta_data_table,
    ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    Ok(())
}
