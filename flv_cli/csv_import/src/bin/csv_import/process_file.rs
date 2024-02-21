use proton_client::ProtonClient;
use std::error::Error;
use std::path::PathBuf;

pub fn process(
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

    // Create a Tokio runtime to wrap the asynchronous code
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Count the number of rows in the file
    let count_query = generate_count_query(path);
    let binding = client.clone();
    let fut = binding.fetch_one(&count_query);
    let number_of_rows: u64 = rt.block_on(fut).expect("Failed to count inserted data");

    // Insert trade data into the database
    let query = generate_insert_query(&file, path);
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
    let query = generate_insert_meta_data_query(
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

fn generate_count_query(path: &str) -> String {
    // SELECT count(*) FROM file('test.csv', 'CSV', 'timestamp int64, price float64, volume float64')
    // https://clickhouse.com/docs/en/sql-reference/table-functions/file#select-from-a-csv-file
    // DateTime64 3 (milliseconds)
    // https://clickhouse.com/docs/en/sql-reference/data-types/datetime64
    format!(
        "SELECT count(*) FROM file('{path}', 'CSV', 'timestamp DateTime64(3), price float64, volume float64')"
    )
}

fn generate_insert_query(file: &str, path: &str) -> String {
    let table_name = format!("KRAKEN_{}", file).to_lowercase();
    //  INSERT INTO stream AS SELECT * FROM file('test.csv', 'CSV', 'timestamp int64, price float64, volume float64')
    // https://clickhouse.com/docs/en/sql-reference/table-functions/file#select-from-a-csv-file
    format!(
        "INSERT INTO {table_name} AS SELECT * FROM file('{path}', 'CSV', 'timestamp DateTime64(3), price float64, volume float64')"
    )
}

fn generate_insert_meta_data_query(
    meta_data_table: &str,
    _table_name: &str,
    _symbol: &str,
    _symbol_id: u64,
    _number_of_rows: u64,
) -> String {
    format!(
        "INSERT INTO {meta_data_table}  'timestamp DateTime64(3), price float64, volume float64'"
    )
}
