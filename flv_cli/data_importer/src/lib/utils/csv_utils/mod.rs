use chrono::{TimeZone, Utc};
use common::prelude::TradeBar;
use csv::ReaderBuilder;
use encoding_rs::UTF_8;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::error::Error;
use std::fs;
use std::path::Path;

/// Reads a CSV file from the given path into a Vec of TradeBar objects.
///
/// # Parameters
///
/// * `path` - The path to the CSV file to read
///
/// # Returns
///
/// A Result containing a Vec of TradeBar objects parsed from the CSV file,
/// or a Box<Error> if there was an issue reading it.
///
/// # Errors
///
/// Panics if the file does not exist, with a formatted error message.
///
/// # Remarks
///
/// - Checks if the file exists before trying to read it
/// - Calls read_csv_lines() to parse the file contents into TradeBar objects
pub fn read_csv_file(path: &str) -> Result<Vec<TradeBar>, Box<dyn Error>> {
    if !Path::new(&path).exists() {
        panic!(
            "{}",
            format!("[csv_utils/read_csv_file]: File {} does not exist", path)
        );
    }

    read_csv_lines(path)
}

/// Reads CSV data from the given file path into a Vec of TradeBar objects.
///
/// # Parameters
///
/// * `path` - The path to the CSV file to read
///
/// # Returns
///
/// A Result containing a Vec of TradeBar objects parsed from the CSV data,
/// or a Box<Error> if there was an issue reading or parsing the file.
///
/// # Remarks
///
/// - Uses the csv crate to parse the CSV data
/// - Assumes the CSV has headers and is semicolon delimited
/// - Pre-allocates the Vec to avoid re-allocations
/// - Decodes the file contents as UTF-8
/// - Prints any CSV parsing errors but continues processing
fn read_csv_lines<P>(path: P) -> Result<Vec<TradeBar>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = fs::read(path).expect("[csv_utils/read_csv_lines]: Could not read file");
    let (res, _, _) = UTF_8.decode(&file);

    let mut content: Vec<TradeBar> = Vec::with_capacity(500_000); // fixed pre-allocation

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(res.as_bytes());

    // https://docs.rs/csv/latest/csv/struct.Reader.html
    for result in rdr.records() {
        match result {
            Ok(record) => {
                let timestamp = record[0]
                    .parse::<i64>()
                    .expect("[csv_utils/read_csv_lines]: Could not parse timestamp");

                let p = record[1]
                    .parse::<f64>()
                    .expect("[csv_utils/read_csv_lines]: Could not parse price");

                let v = record[2]
                    .parse::<f64>()
                    .expect("[csv_utils/read_csv_lines]: Could not parse volume");

                let date_time = Utc.timestamp_opt(timestamp, 0).unwrap();

                let price = Decimal::from_f64(p)
                    .expect("[csv_utils/read_csv_lines]: Could not parse price from f64");

                let volume = Decimal::from_f64(v)
                    .expect("[csv_utils/read_csv_lines]: Could not parse volume from f64");

                let trade_bar = TradeBar::new(date_time, price, volume);

                content.push(trade_bar);
            }
            Err(err) => {
                println!(
                    "[csv_utils/read_csv_lines]: error reading CSV line: {}",
                    err
                );
            }
        }
    }

    Ok(content)
}
