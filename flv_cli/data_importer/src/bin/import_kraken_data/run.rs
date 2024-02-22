use db_quest_manager::QuestDBManager;
use client_utils::{config_utils, csv_utils, file_utils, print_utils};
use std::error::Error;
use std::time::Instant;

const CONFIG_FILE_NAME: &str = "import_config.toml";
const META_DATA_TABLE: &str = "kraken_symbols";

/// Runs the Kraken data import process.
///
/// # Steps:
///
/// - Initialize start time and verbosity flag
/// - Load import config
/// - Get database config
/// - Connect to QuestDB
/// - Get all CSV files in data folder
/// - Loop through each file:
///   - Parse CSV to TradeBars
///   - Insert TradeBars into QuestDB table
///   - Update imported files count
/// - Print output:
///   - Total files vs imported
///   - Duration
///
/// # Returns
///
/// `Result<(), Box<dyn Error>>`
///
pub fn run() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let vrb = true;

    print_utils::print_import_header();

    print_utils::dbg_print(vrb, "Build import config");
    let config =
        config_utils::get_config_file(CONFIG_FILE_NAME).expect("Import config file not found");

    print_utils::dbg_print(
        vrb,
        format!("Import data folder: {}", config.data_folder()).as_str(),
    );

    // Get DB config
    print_utils::dbg_print(vrb, "Build DB config");
    let db_config = config_utils::get_local_db_config();
    println!("DB config: {:?}", db_config);
    println!();

    // Connect to QuestDB
    print_utils::dbg_print(vrb, "Connect to QuestDB");
    let mut db_manager = QuestDBManager::new(db_config);

    print_utils::dbg_print(vrb, "Read all files in data folder");
    let files = file_utils::get_file_paths_from_directory(config.data_folder())
        .expect("Failed to read files in data folder");

    print_utils::dbg_print(vrb, format!("Found {} files", files.len()).as_str());

    print_utils::dbg_print(vrb, "Import all data files into Quest DB");
    let mut imported_files = 1;

    for file_path in &files {
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
            break;
        }

        let table_name = format!("KRAKEN_{}", file).to_lowercase();
        let symbol = file.to_lowercase();
        let symbol_id = imported_files as i64;

        db_manager
            .insert_trade_bars(trade_bars, &table_name, &symbol, symbol_id, META_DATA_TABLE)
            .expect("Failed to insert trade data bars into DB");

        imported_files += 1;
    }

    println!();
    print_utils::dbg_print(
        vrb,
        format!("Imported {} files out of {}", imported_files, files.len()).as_str(),
    );

    print_utils::print_duration(&start.elapsed());

    Ok(())
}
