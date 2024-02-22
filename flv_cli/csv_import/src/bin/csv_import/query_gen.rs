pub(crate) fn generate_count_query(path: &str) -> String {
    // SELECT count(*) FROM file('test.csv', 'CSV', 'timestamp int64, price float64, volume float64')
    // https://clickhouse.com/docs/en/sql-reference/table-functions/file#select-from-a-csv-file
    // DateTime64 3 (milliseconds)
    // https://clickhouse.com/docs/en/sql-reference/data-types/datetime64
    format!(
        "SELECT count(*) FROM file('{path}', 'CSV', 'timestamp DateTime64(3), price float64, volume float64')"
    )
}

pub(crate) fn generate_insert_query(file: &str, path: &str) -> String {
    let table_name = format!("KRAKEN_{}", file).to_lowercase();
    //  INSERT INTO stream AS SELECT * FROM file('test.csv', 'CSV', 'timestamp int64, price float64, volume float64')
    // https://clickhouse.com/docs/en/sql-reference/table-functions/file#select-from-a-csv-file
    format!(
        "INSERT INTO {table_name} AS SELECT * FROM file('{path}', 'CSV', 'timestamp DateTime64(3), price float64, volume float64')"
    )
}

pub(crate) fn generate_insert_meta_data_query(
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
