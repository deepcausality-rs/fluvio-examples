use common::prelude::DBConfig;

const PORT: u16 = 8000;
const DB_NAME: &str = "fluvio_db";
const NAMESPACE: &str = "datastream";

pub fn db_config_local() -> DBConfig {
    DBConfig::new(
        PORT,
        "0.0.0.0".to_string(),
        DB_NAME.to_string(),
        NAMESPACE.to_string(),
        "root".to_string(),
        "root".to_string(),
    )
}

pub fn db_config_cluster() -> DBConfig {
    DBConfig::new(
        PORT,
        "surrealdb.namespace.url.cluster".to_string(),
        DB_NAME.to_string(),
        NAMESPACE.to_string(),
        "root".to_string(),
        "root".to_string(),
    )
}
