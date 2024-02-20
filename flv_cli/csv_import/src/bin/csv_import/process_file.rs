use proton_client::ProtonClient;
use std::error::Error;
use std::path::PathBuf;
use proton_client::prelude::ProtonClientError;

pub async fn process(
    client: &ProtonClient,
    file_path: &PathBuf,
    symbol_id: i64,
    meta_data_table: &str,
) -> Result<(), Box<dyn Error>> {
    // get file name without extension
    let file = file_path
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert file name to str")
        .replace(".csv", "");

    // get file path
    let path = file_path
        .to_str()
        .expect("Failed to convert file path to str");


    // let query = generate_insert_query(&file, path);
    //
    // println!("Query: {}", query);

    let count_query = generate_count_query(path);

    println!("Count Query: {}", count_query);

    let count = select_count(client, path)
        .await
        .expect("[main/count]: Failed to count inserted data");

    println!(" Inserted data count: {}", count);

    // // Construct some fields
    // let number_of_rows = 0_i64;
    // let table_name = format!("KRAKEN_{}", file).to_lowercase();
    // let symbol = file.to_lowercase();
    // let symbol_id = imported_files;


    // insert meta data into Proton meta data table

    Ok(())
}

async fn select_count(client: &ProtonClient, path: &str) -> Result<u64, ProtonClientError> {
    let count_query = generate_count_query(path);

    let count = client
        .clone()
        .fetch_one(&count_query)
        .await
        .expect("[main/select_count]: Failed to fetch count()");

    Ok(count)
}

pub async fn insert(client: &ProtonClient, file: &str, path: &str) -> Result<(), ProtonClientError> {
    let query = generate_insert_query(&file, path);

    let res = client.execute_query(&query).await;

    if res.is_err() {
        let err = res.err().unwrap();
        println!("[main/insert]: Error: {}", err);
        return Err(err);
    }

    Ok(())
}


fn generate_insert_query(file: &str, path: &str) -> String {
    let table_name = format!("KRAKEN_{}", file).to_lowercase();
    //  INSERT INTO stream AS SELECT * FROM file('test.csv', 'CSV', 'timestamp int64, price float64, volume float64')
    // https://clickhouse.com/docs/en/sql-reference/table-functions/file#select-from-a-csv-file
    format!("INSERT INTO {table_name} AS SELECT * FROM file('{path}', 'CSV', \
    'timestamp int64, price float64, volume float64')")
}

fn generate_count_query(path: &str) -> String {
    // SELECT count(*) FROM file('test.csv', 'CSV', 'timestamp int64, price float64, volume float64')
    // https://clickhouse.com/docs/en/sql-reference/table-functions/file#select-from-a-csv-file
    format!("SELECT count(*) FROM file('{path}', 'CSV', 'timestamp int64, price float64, volume float64')")
}