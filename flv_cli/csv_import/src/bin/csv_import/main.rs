use client_utils::prelude::{config_utils, file_utils, print_utils};
use std::time::Instant;
use clickhouse::Client;
use crate::process_file::create_meta_data_table;

mod meta_data;
mod process_file;
mod query_gen;

const CONFIG_FILE_NAME: &str = "import_config.toml";
const META_DATA_TABLE: &str = "kraken_symbols";
const VERBOSE: bool = true;

fn main() {
    let start = Instant::now();
    print_utils::print_import_header();
    // Enables verbose output for main
    let vrb = VERBOSE;
    // Enables verbose output for process_file.
    let vrb_prc = false;

    print_utils::dbg_print(vrb, "Build Proton Client");
    let client = Client::default().with_url("http://localhost:8123");

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

    print_utils::dbg_print(vrb, "Build a Tokio runtime to wrap the asynchronous code");
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    print_utils::dbg_print(vrb, "Build metadata table");
    let res = rt.block_on(create_meta_data_table(&client, META_DATA_TABLE));
    if res.is_err() {
        println!("[main]: Failed to create metadata table. Please check the error, check the generated DDL and try again.");
        panic!("{:?}", res.err().unwrap());
    }

    println!("Importing files");
    // Sequential iterator requires a simple mutable counter
    let mut symbol_id = 0;
    // Iterate over the files
    for file_path in &files {
        symbol_id += 1;
        process_file::process(
            &rt,
            &client,
            file_path,
            symbol_id,
            META_DATA_TABLE,
            vrb_prc,
        ).expect("Failed to import file");
    }

    println!();
    print_utils::dbg_print(
        vrb,
        format!("Imported {} files out of {}", symbol_id, files.len()).as_str(),
    );

    print_utils::print_duration(&start.elapsed());
}
