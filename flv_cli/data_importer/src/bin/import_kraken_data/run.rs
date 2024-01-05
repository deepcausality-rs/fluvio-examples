use db_quest_manager::QuestDBManager;
use lib_import::utils::{config_utils, csv_utils, file_utils, print_utils};
use std::error::Error;
use std::time::Instant;

const CONFIG_FILE_NAME: &str = "import_config.toml";
const META_DATA_TABLE: &str = "kraken_symbols";

pub fn run() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let vrb = true;

    print_utils::print_import_header();

    print_utils::dbg_print(vrb, "Build import config");
    let config = match config_utils::get_config_file(CONFIG_FILE_NAME) {
        Ok(config) => config,
        Err(e) => return Err(e),
    };
    print_utils::dbg_print(
        vrb,
        format!("Import data folder: {}", config.data_folder()).as_str(),
    );

    print_utils::dbg_print(vrb, "Build DB config");
    let db_config = config_utils::get_local_db_config();
    println!("DB config: {:?}", db_config);
    println!();

    print_utils::dbg_print(vrb, "Connect to QuestDB");
    let mut db_manager = QuestDBManager::new(db_config);

    print_utils::dbg_print(vrb, "Read all files in data folder");
    let files = match file_utils::get_file_paths_from_directory(config.data_folder()) {
        Ok(files) => files,
        Err(e) => return Err(Box::from(e)),
    };
    print_utils::dbg_print(vrb, format!("Found {} files", files.len()).as_str());

    print_utils::dbg_print(vrb, "Import all data files into DB");
    let mut imported_files = 1;

    for file_path in &files {
        let file = file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".csv", "");

        let trade_bars = csv_utils::read_csv_file(file_path.to_str().unwrap());
        assert!(trade_bars.is_ok());

        let trade_bars = trade_bars.unwrap();

        if trade_bars.is_empty() {
            break;
        }

        let table_name = format!("KRAKEN_{}", file).to_lowercase();
        let symbol = file.to_lowercase();
        let symbol_id = imported_files as i64;

        let result = db_manager.insert_trade_bars(
            trade_bars,
            &table_name,
            &symbol,
            symbol_id,
            META_DATA_TABLE,
        );

        assert!(result.is_ok());

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
