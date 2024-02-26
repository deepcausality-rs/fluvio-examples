use common::prelude::ClickHouseConfig;

pub fn get_local_db_config() -> ClickHouseConfig {
    ClickHouseConfig::default()
}

pub fn get_cluster_db_config() -> ClickHouseConfig {
    ClickHouseConfig::new(
        "http://clickhouse.default.svc.cluster.local".to_string(),
        8123,
        "username".to_string(),
        "password".to_string(),
        "default".to_string(),
    )
}
