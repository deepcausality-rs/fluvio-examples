use crate::query_gen;
use lib_csv_import::types::meta_data::MetaData;
use lib_csv_import::utils::print_utils;
use proton_client::ProtonClient;
use std::error::Error;
use std::path::PathBuf;
use tokio::runtime::Runtime;

pub fn process(
    rt: &Runtime,
    client: &ProtonClient,
    file_path: &PathBuf,
    symbol_id: u64,
    meta_data_table: &str,
    vrb: bool,
) -> Result<(), Box<dyn Error>> {
    print_utils::dbg_print(vrb, "Get file name without extension");
    let file = file_path
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert file name to str")
        .replace(".csv", "");

    print_utils::dbg_print(vrb, "Get file path");
    let path = file_path
        .to_str()
        .expect("Failed to convert file path to str");

    print_utils::dbg_print(vrb, "Construct meta data fields");
    let table_name = format!("KRAKEN_{}", file).to_lowercase();
    let symbol = file.to_lowercase();

    print_utils::dbg_print(vrb, "Count number of rows in file");
    let count_query = query_gen::generate_count_query(path);
    let binding = client.clone();
    let fut = binding.fetch_one(&count_query);
    let number_of_rows: u64 = rt.block_on(fut).expect("Failed to count inserted data");

    print_utils::dbg_print(
        vrb,
        "Create the trade data table if it doesn't exist in the database",
    );
    let query = query_gen::generate_trade_table_ddl(&table_name);
    let binding = client.clone();
    let fut = binding.execute_query(&query);
    let res = rt.block_on(fut);
    // Check for error
    if res.is_err() {
        println!("[main/insert]: Failed to create trade table");
        return Err(Box::try_from(res.err().unwrap()).unwrap());
    }

    print_utils::dbg_print(vrb, "Insert trade data into the trade table");
    let query = query_gen::generate_insert_query(&file, path);
    let binding = client.clone();
    let fut = binding.execute_query(&query);
    let res = rt.block_on(fut);
    // Check for error
    if res.is_err() {
        println!("[main/insert]: Failed to insert trade data into DB");
        return Err(Box::try_from(res.err().unwrap()).unwrap());
    }

    print_utils::dbg_print(vrb, "Insert meta data into Proton meta data table");
    let meta_data = MetaData::new(&table_name, &symbol, symbol_id, number_of_rows);
    let binding = client.clone();
    let fut = binding.insert(meta_data_table);
    let res = rt.block_on(fut);
    // Check for error
    if res.is_err() {
        println!("[main/insert]: Failed to get inserter for meta data");
        return Err(Box::try_from(res.err().unwrap()).unwrap());
    }
    // Write meta data into Proton meta data table
    let mut insert = res.unwrap();
    let res = rt.block_on(insert.write(&meta_data));
    // Check for error
    if res.is_err() {
        println!("[main/insert]: Failed to insert meta data into DB");
        return Err(Box::try_from(res.err().unwrap()).unwrap());
    }

    Ok(())
}
