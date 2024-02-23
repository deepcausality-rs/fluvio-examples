pub(crate) fn generate_trade_table_ddl(table_name: &str) -> String {
    // CREATE STREAM default.kraken_1incheur
    // (
    //   `timestamp` datetime64(3),
    //   `price` float64,
    //   `volume` float64,
    //   `_tp_time` datetime64(3, 'UTC') DEFAULT timestamp CODEC(DoubleDelta, LZ4),
    //   INDEX _tp_time_index _tp_time TYPE minmax GRANULARITY 2
    // )
    // ENGINE = Stream(1, 1, rand())
    // PARTITION BY to_YYYYMM(timestamp)
    // ORDER BY to_start_of_hour(_tp_time)
    // SETTINGS event_time_column = 'timestamp', index_granularity = 8192"
    format!(
        r"CREATE STREAM IF NOT EXISTS {table_name}
        (
           `timestamp` datetime64(3),
           `price` float64,
           `volume` float64,
           `_tp_time` datetime64(3, 'UTC') DEFAULT timestamp CODEC(DoubleDelta, LZ4),
            INDEX _tp_time_index _tp_time TYPE minmax GRANULARITY 2
        )
     ENGINE = Stream(1, 1, rand())
     PARTITION BY to_YYYYMM(timestamp)
     ORDER BY to_start_of_hour(_tp_time)
     SETTINGS event_time_column = 'timestamp', index_granularity = 8192"
    )
}

pub(crate) fn generate_insert_query(file: &str, path: &str) -> String {
    let table_name = format!("KRAKEN_{}", file).to_lowercase();
     // INSERT INTO kraken_1incheur (timestamp, price, volume) SELECT
    //   timestamp, price, volume
    // FROM
    //  file('1INCHEUR.csv', 'CSV', 'timestamp datetime64(3), price float64, volume float64')
     format!(
        r"INSERT INTO {table_name} (timestamp, price, volume)
        SELECT timestamp, price, volume
        FROM
        file('{path}', 'CSV', 'timestamp DateTime64(3), price float64, volume float64')"
    )
}


pub(crate) fn generate_count_query(path: &str) -> String {
    format!(
        "SELECT count(*) FROM file('{path}', 'CSV', 'timestamp datetime64(3), price float64, volume float64')"
    )
}

pub(crate) fn generate_metadata_table_ddl(meta_data_table: &str) -> String {
    format!(
        r"CREATE STREAM IF NOT EXISTS {meta_data_table} (
            symbol string,
            symbol_id uint64,
            table_name string,
            number_of_rows uint64
        )
        ORDER BY symbol
        primary key symbol",
    )
}
