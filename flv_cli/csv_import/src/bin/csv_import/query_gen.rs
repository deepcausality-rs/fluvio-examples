pub(crate) fn generate_trade_table_ddl(table_name: &str) -> String {
    format!(
        "CREATE STREAM IF NOT EXISTS {table_name} \
        (timestamp datetime64(3), price float64,  volume float64) \
        SETTINGS event_time_column='timestamp'"
    )
}

pub(crate) fn generate_count_query(path: &str) -> String {
    // SELECT count(*) FROM file('test.csv', 'CSV', 'timestamp int64, price float64, volume float64')
    // https://clickhouse.com/docs/en/sql-reference/table-functions/file#select-from-a-csv-file
    format!(
        "SELECT count(*) FROM file('{path}', 'CSV', 'timestamp datetime64(3), price float64, volume float64')"
    )
}

pub(crate) fn generate_insert_query(file: &str, path: &str) -> String {
    let table_name = format!("KRAKEN_{}", file).to_lowercase();
    //  INSERT INTO stream AS SELECT * FROM file('test.csv', 'CSV', 'timestamp int64, price float64, volume float64')
    // https://clickhouse.com/docs/en/sql-reference/table-functions/file#select-from-a-csv-file
    format!(
        "INSERT INTO {table_name} SELECT * FROM \
        file('{path}', 'CSV', 'timestamp datetime64(3), price float64, volume float64')"
    )
}

pub(crate) fn generate_metadata_table_ddl(meta_data_table: &str) -> String {
    format!(
        "CREATE STREAM IF NOT EXISTS {meta_data_table} ( \
            symbol string, \
            symbol_id uint64, \
            table_name string, \
            number_of_rows uint64 \
        ) ENGINE = MergeTree() \
        ORDER BY symbol \
        primary key symbol",
    )
}
