mod process_file;
mod query_gen;
mod query_utils;
mod types;

use client_utils::prelude::{config_utils, file_utils, print_utils};
use common::prelude::ClickHouseConfig;
use klickhouse::{Client, ClientOptions};
use query_utils::create_meta_data_table;
use std::time::Instant;

const CONFIG_FILE_NAME: &str = "import_config.toml";
const META_DATA_TABLE: &str = "kraken_symbols";
const VERBOSE: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    print_utils::print_import_header();
    // Enables verbose output for main
    let vrb = VERBOSE;
    // Enables verbose output for process_file.
    let vrb_prc = false;

    print_utils::dbg_print(vrb, "Build DB Client");
    let db_config = ClickHouseConfig::default();
    let destination = db_config.connection_string();
    let client = Client::connect(destination.clone(), ClientOptions::default())
        .await
        .expect(format!("Failed to connect to {}", &destination).as_str());

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

    print_utils::dbg_print(vrb, "Build metadata table");
    create_meta_data_table(&client, META_DATA_TABLE)
        .await
        .expect("Failed to create metadata table");

    println!("Importing files");
    // Sequential iterator requires a simple mutable counter
    let mut symbol_id = 0;
    // Iterate over the files
    for file_path in &files {
        symbol_id += 1;
        process_file::process(&client, file_path, symbol_id, META_DATA_TABLE, vrb_prc)
            .await
            .expect("Failed to import file");
    }

    println!();
    print_utils::dbg_print(
        vrb,
        format!("Imported {} files out of {}", symbol_id, files.len()).as_str(),
    );

    print_utils::print_duration(&start.elapsed());
    Ok(())
}
