use crate::process_file;
use lib_csv_import::utils::{config_utils, file_utils, print_utils};
use rayon::prelude::*;

use std::error::Error;
use std::time::Instant;

const CONFIG_FILE_NAME: &str = "import_config.toml";
const META_DATA_TABLE: &str = "kraken_symbols";

/// Executes the CSV import process.
///
/// This function orchestrates the import of CSV files into the database. It initializes the
/// necessary components, reads the import configuration, and processes each file found in the
/// specified data folder. Files can be processed either in parallel or sequentially based on
/// the configuration.
///
/// # Errors
///
/// This function will return an error if:
/// - The import configuration file is not found.
/// - There is a failure reading files from the data folder.
/// - There is a failure during the processing of individual files.
///
/// # Panics
///
/// This function may panic if there is an unexpected error during the import process, such as
/// an issue with the underlying file system or a problem with the database connection.
///
/// ```
///
pub async fn run() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let vrb = true;

    print_utils::dbg_print(vrb, "Build Proton Client");
    let client = proton_client::prelude::ProtonClient::default();

    print_utils::print_import_header();

    print_utils::dbg_print(vrb, "Build import config");
    let config =
        config_utils::get_config_file(CONFIG_FILE_NAME).expect("Import config file not found");

    print_utils::dbg_print(
        vrb,
        format!("Import data folder: {}", config.data_folder()).as_str(),
    );

    print_utils::dbg_print(vrb, "Read all files in data folder");
    let files = file_utils::get_file_paths_from_directory(config.data_folder())
        .expect("Failed to read files in data folder");

    print_utils::dbg_print(vrb, format!("Found {} files", files.len()).as_str());

    let mut symbol_id = 0;

    if config.parallel() {
        println!("Importing files in parallel");
        // Parallel iterator requires an atomic counter for thread safety
        let counter = client_utils::atomic_counter::RelaxedAtomicCounter::new();

        // files.par_iter().for_each(async move |file_path| {
        //     process_file::process(
        //         &client,
        //         file_path,
        //         counter.increment_and_get() as i64,
        //         META_DATA_TABLE,
        //     ).await
        //     .expect("Failed to import file")
        // });

        symbol_id = counter.get_counter() as i64;
    } else {
        println!("Importing files in sequence");
        for file_path in &files {
            // Sequential iterator requires only a simple mutable counter
            symbol_id += 1;
            process_file::process(&client, file_path, symbol_id, META_DATA_TABLE)
                .await
                .expect("Failed to import file");
        }
    }

    println!();
    print_utils::dbg_print(
        vrb,
        format!("Imported {} files out of {}", symbol_id, files.len()).as_str(),
    );

    print_utils::print_duration(&start.elapsed());

    Ok(())
}
