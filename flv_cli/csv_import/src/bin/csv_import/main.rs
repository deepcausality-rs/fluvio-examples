use lib_csv_import::utils::{config_utils, file_utils, print_utils};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Instant;

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
    let vrb_prc = VERBOSE;

    print_utils::dbg_print(vrb, "Build Proton Client");
    let client = proton_client::prelude::ProtonClient::default();

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

    print_utils::dbg_print(vrb, "Build a Tokio runtime to wrap the asynchronous code");
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    print_utils::dbg_print(vrb, "Build metadata table");
    let query = query_gen::generate_metadata_table_ddl(META_DATA_TABLE);
    let binding = client.clone();
    let fut = binding.execute_query(&query);
    let res = rt.block_on(fut);
    // Check for error
    if res.is_err() {
        println!("[main]: Failed to create metadata table. Please check the error, check the generated DDL and try again.");
        panic!("{:?}", res.err().unwrap());
    }

    if config.parallel() {
        println!("Importing files in parallel");
        // Parallel iterator requires an atomic counter for thread safety
        let counter = client_utils::atomic_counter::RelaxedAtomicCounter::new();

        files.par_iter().for_each(|file_path| {
            process_file::process(
                &rt,
                &client,
                file_path,
                counter.increment_and_get(),
                META_DATA_TABLE,
                vrb_prc,
            )
            .expect("Failed to import file")
        });

        symbol_id = counter.get_counter();
    } else {
        println!("Importing files in sequence");
        for file_path in &files {
            // Sequential iterator requires only a simple mutable counter
            symbol_id += 1;
            process_file::process(&rt, &client, file_path, symbol_id, META_DATA_TABLE, vrb_prc)
                .expect("Failed to import file");
        }
    }

    println!();
    print_utils::dbg_print(
        vrb,
        format!("Imported {} files out of {}", symbol_id, files.len()).as_str(),
    );

    print_utils::print_duration(&start.elapsed());
}
