pub(crate) fn generate_trade_table_ddl(table_name: &str) -> String {
    format!(
        "CREATE STREAM IF NOT EXISTS {table_name} \
        (\
        timestamp datetime64(3, 'UTC'), \
        price float64,  \
        volume float64\
        ) \
        SETTINGS event_time_column='timestamp'"
    )
}

pub(crate) fn generate_count_query(path: &str) -> String {
    format!(
        "SELECT count(*) FROM file('{path}', 'CSV', 'timestamp datetime64(3), price float64, volume float64')"
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
