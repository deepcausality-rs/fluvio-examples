use crate::meta_data::MetaData;
use crate::{query_gen};
use client_utils::print_utils;
use std::error::Error;
use std::path::PathBuf;
use clickhouse::Client;
use tokio::runtime::Runtime;


/// Process a CSV file for import into TimePlus Proton.
///
/// # Arguments
///
/// * `rt` - Tokio runtime
/// * `client` - Proton client
/// * `file_path` - Path to CSV file
/// * `symbol_id` - Unique symbol ID
/// * `meta_data_table` - Name of Proton meta data table
/// * `vrb` - Verbose boolean flag
///
/// # The process:
/// * Extracts the filename and path from the input file path
/// * Generates a SQL query to count the number of rows in the CSV file
/// * Executes the count query and saves the number of rows
/// * Generates a SQL CREATE TABLE statement for a new table to hold the CSV data
/// * Executes the CREATE TABLE statement
/// * Generates a SQL INSERT statement to populate the new table from the CSV
/// * Executes the INSERT statement
/// * Creates a MetaData struct with metadata about the imported CSV
/// * Inserts the MetaData into the Proton metadata table
///
/// # Returns
///
/// Returns Ok() on success, or an error if any step fails.
///
/// # Errors
///
/// Will return an error if:
///
/// - Failed to get file name
/// - Failed to convert file name to string
/// - Failed to convert file path to string
/// - Failed to count rows in CSV file
/// - Failed to create trade table in QuestDB
/// - Failed to insert trade data into QuestDB
/// - Failed to get Proton inserter for meta data
/// - Failed to insert meta data into Proton
///
pub fn process(
    rt: &Runtime,
    client: &Client,
    file_path: &PathBuf,
    symbol_id: u32,
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
    let number_of_rows: u64 = rt
        .block_on(count_rows(&client, path))
        .expect("Failed to count inserted data");
    if vrb {
        println!("Number of rows: {}", number_of_rows);
    }

    print_utils::dbg_print(vrb, "Create the trade data table if it doesn't exist");
    rt.block_on(create_trade_data_table(&client, &table_name)).expect("Failed to create trade table");

    print_utils::dbg_print(vrb, "Insert trade data into the trade table");
    rt.block_on(insert_trade_data(&client, &file, path)).expect("Failed to insert trade data");

    print_utils::dbg_print(vrb, "Insert meta data into meta data table");
    let meta_data = MetaData::new(&table_name, &symbol, symbol_id, number_of_rows);
    rt.block_on(insert_meta_data(&client, &meta_data, meta_data_table)).expect("Failed to insert meta data");

    Ok(())
}

pub(crate) async fn count_rows(client: &Client, path: &str) -> Result<u64, Box<dyn Error>> {
    let count_query = query_gen::generate_count_query(path);
    let number_of_rows: u64 = client
        .query(&count_query)
        .fetch_one()
        .await
        .expect("Failed to count rows in CSV file");

    Ok(number_of_rows)
}

pub(crate) async fn create_trade_data_table(client: &Client, table_name: &str) -> Result<(), Box<dyn Error>> {
    let query = query_gen::generate_trade_table_ddl(table_name);
    client
        .query(&query)
        .execute()
        .await
        .expect("[main/create_trade_data_table]: Failed to create trade table");

    Ok(())
}

pub(crate) async fn insert_trade_data(client: &Client, file: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let query = query_gen::generate_insert_query(&file, &path);
    client
        .query(&query)
        .execute()
        .await
        .expect("Failed to insert data");

    Ok(())
}

pub(crate) async fn create_meta_data_table(client: &Client, meta_data_table: &str) -> Result<(), Box<dyn Error>> {
    let query = query_gen::generate_metadata_table_ddl(meta_data_table);
    client
        .query(&query)
        .execute()
        .await
        .expect("[main/create_meta_data_table]: Failed to create meta data table");

    Ok(())
}

pub(crate) async fn insert_meta_data(client: &Client, meta_data: &MetaData<'_>, meta_data_table: &str) -> Result<(), Box<dyn Error>> {
    let mut insert = client.inserter(meta_data_table).unwrap();

    insert.write(meta_data).await.expect("Failed to write meta data");

    insert.end().await.expect("Failed to end insert");

    Ok(())
}