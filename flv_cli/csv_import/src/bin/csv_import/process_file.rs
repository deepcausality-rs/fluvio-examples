use lib_csv_import::utils::csv_utils;
use std::error::Error;
use std::path::PathBuf;

pub fn process(file_path: &PathBuf, imported_files: i64) -> Result<(), Box<dyn Error>> {
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

    // let table_name = format!("KRAKEN_{}", file).to_lowercase();
    // let symbol = file.to_lowercase();
    // let symbol_id = imported_files;

    Ok(())
}
