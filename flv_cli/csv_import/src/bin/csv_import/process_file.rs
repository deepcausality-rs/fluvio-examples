use crate::query_gen;
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
) -> Result<(), Box<dyn Error>> {
    // Get file name without extension
    let file = file_path
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert file name to str")
        .replace(".csv", "");

    // Get file path
    let path = file_path
        .to_str()
        .expect("Failed to convert file path to str");

    // Count the number of rows in the file
    let count_query = query_gen::generate_count_query(path);
    let binding = client.clone();
    let fut = binding.fetch_one(&count_query);
    let number_of_rows: u64 = rt.block_on(fut).expect("Failed to count inserted data");

    // Insert trade data into the database
    let query = query_gen::generate_insert_query(&file, path);
    let binding = client.clone();
    let fut = binding.execute_query(&query);
    let res = rt.block_on(fut);

    // Check for error
    if res.is_err() {
        println!("[main/insert]: Failed to insert trade data into DB");
        return Err(Box::try_from(res.err().unwrap()).unwrap());
    }

    // Construct some fields
    let table_name = format!("KRAKEN_{}", file).to_lowercase();
    let symbol = file.to_lowercase();

    // Build query to insert meta data
    let query = query_gen::generate_insert_meta_data_query(
        meta_data_table,
        &table_name,
        &symbol,
        symbol_id,
        number_of_rows,
    );

    // Insert meta data into Proton meta data table
    let binding = client.clone();
    let fut = binding.execute_query(&query);
    let res = rt.block_on(fut);

    // Check for error
    if res.is_err() {
        println!("[main/insert]: Failed to insert meta data into DB");
        return Err(Box::try_from(res.err().unwrap()).unwrap());
    }

    Ok(())
}
