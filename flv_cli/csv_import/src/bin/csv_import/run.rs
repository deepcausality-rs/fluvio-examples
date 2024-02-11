use crate::process_file;
use lib_csv_import::utils::{config_utils, file_utils, print_utils};
use rayon::prelude::*;

use std::error::Error;
use std::time::Instant;

const CONFIG_FILE_NAME: &str = "import_config.toml";
const META_DATA_TABLE: &str = "kraken_symbols";

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

    print_utils::dbg_print(vrb, "Read all files in data folder");
    let files = file_utils::get_file_paths_from_directory(config.data_folder())
        .expect("Failed to read files in data folder");

    print_utils::dbg_print(vrb, format!("Found {} files", files.len()).as_str());

    let mut imported_files = 1;

    if config.parallel() {
        println!("Importing files in parallel");
        files.par_iter()
            .for_each(|file_path|
                process_file::process(file_path, imported_files)
                    .expect("Failed to import file")
            );

        imported_files = files.len() as i64;
    } else {
        println!("Importing files in sequence");
        for file_path in &files {
            process_file::process(file_path, imported_files)
                .expect("Failed to import file");
            ;
            imported_files += 1;
        }
    }

    println!();
    print_utils::dbg_print(
        vrb,
        format!("Imported {} files out of {}", imported_files, files.len()).as_str(),
    );

    print_utils::print_duration(&start.elapsed());

    Ok(())
}
