use crate::prelude::TradeBar;
use csv::ReaderBuilder;
use encoding_rs::UTF_8;
use std::fmt::Error;
use std::fs;
use std::path::Path;

const SEMICOLON_DELIMITER: u8 = b';';

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
pub fn read_csv_file(path: &str) -> Result<Vec<TradeBar>, Box<Error>> {
    if !Path::new(&path).exists() {
        panic!("{}", format!("File {} does not exist", path));
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
fn read_csv_lines<P>(path: P) -> Result<Vec<TradeBar>, Box<Error>>
where
    P: AsRef<Path>,
{
    let file = fs::read(path).expect("Could not read file");
    let (res, _, _) = UTF_8.decode(&file);
    let mut content: Vec<TradeBar> = Vec::with_capacity(500_000); // fixed pre-allocation

    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .double_quote(true)
        .delimiter(SEMICOLON_DELIMITER)
        .from_reader(res.as_bytes());

    for result in rdr.records() {
        match result {
            Ok(record) => {
                let row: TradeBar = record.deserialize(None).expect("Invalid CSV format");
                content.push(row);
            }
            Err(err) => {
                println!("error reading CSV line: {}", err);
            }
        }
    }

    Ok(content)
}
