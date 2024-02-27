pub(crate) fn generate_trade_table_ddl(table_name: &str) -> String {
    format!(
        r"
        CREATE TABLE IF NOT EXISTS default.{table_name}
        (
           `timestamp` Datetime64(3) CODEC(DoubleDelta, LZ4),
           `price`  Float64 CODEC(Delta, LZ4),
           `volume` Float64 CODEC(Delta, LZ4),
        )
     ENGINE = MergeTree()
     PRIMARY KEY toStartOfHour(timestamp)
     "
    )
}

pub(crate) fn generate_insert_query(file: &str, path: &str) -> String {
    let table_name = format!("KRAKEN_{}", file).to_lowercase();
    format!(
        r"
        INSERT INTO {table_name} (timestamp, price, volume)
        SELECT timestamp, price, volume
        FROM
        file('{path}', 'CSV', 'timestamp Datetime64(3), price Float64, volume Float64')
        "
    )
}

pub(crate) fn generate_count_query(path: &str) -> String {
    format!(
        r"
    SELECT count(*) FROM file('{path}', 'CSV', 'timestamp Datetime64(3), price Float64, volume Float64')
    "
    )
}

pub(crate) fn generate_metadata_table_ddl(meta_data_table: &str) -> String {
    format!(
        r"
        CREATE TABLE IF NOT EXISTS {meta_data_table}
        (
            symbol String,
            symbol_id UInt64,
            table_name String,
            number_of_rows UInt64
        )
        ENGINE = MergeTree
        PRIMARY KEY (symbol, symbol_id)
        SETTINGS index_granularity=128;
        "
    )
}

pub(crate) fn generate_meta_data_insert_query(meta_data_table: &str) -> String {
    format!(
        r"
    INSERT INTO {meta_data_table} FORMAT native
    "
    )
}
