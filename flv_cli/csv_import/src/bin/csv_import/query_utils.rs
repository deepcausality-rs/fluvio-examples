use crate::query_gen;
use crate::types::{CountRow, MetaData};
use klickhouse::Client;
use std::error::Error;

pub(crate) async fn count_rows(client: &Client, path: &str) -> Result<u64, Box<dyn Error>> {
    let count_query = query_gen::generate_count_query(path);
    let number_of_rows: CountRow = client
        .query_one(&count_query)
        .await
        .expect("Failed to count rows in CSV file");

    Ok(number_of_rows.count())
}

pub(crate) async fn create_trade_data_table(
    client: &Client,
    table_name: &str,
) -> Result<(), Box<dyn Error>> {
    let query = query_gen::generate_trade_table_ddl(table_name);
    client
        .execute(&query)
        .await
        .expect("[create_trade_data_table]: Failed to create trade table");

    Ok(())
}

pub(crate) async fn insert_trade_data(
    client: &Client,
    file: &str,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let query = query_gen::generate_insert_query(&file, &path);
    client.execute(&query).await.expect("Failed to insert data");

    Ok(())
}

pub(crate) async fn create_meta_data_table(
    client: &Client,
    meta_data_table: &str,
) -> Result<(), Box<dyn Error>> {
    let query = query_gen::generate_metadata_table_ddl(meta_data_table);
    client
        .execute(&query)
        .await
        .expect("[create_meta_data_table]: Failed to create meta data table");

    Ok(())
}

pub(crate) async fn insert_meta_data(
    client: &Client,
    meta_data: MetaData,
    meta_data_table: &str,
) -> Result<(), Box<dyn Error>> {
    let query = query_gen::generate_meta_data_insert_query(meta_data_table);
    let rows = vec![meta_data.clone()];

    client
        .insert_native_block(query, rows)
        .await
        .expect("Failed to write meta data");

    Ok(())
}
