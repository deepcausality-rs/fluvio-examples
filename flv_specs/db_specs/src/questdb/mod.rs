use common::prelude::DBConfig;

const PORT: u16 = 9009;
const EXCHANGE_TABLE_NAME: &str = "exchanges";

pub fn get_local_db_config() -> DBConfig {
    DBConfig::new(PORT, "0.0.0.0".into(), EXCHANGE_TABLE_NAME.to_string())
}

pub fn get_cluster_db_config() -> DBConfig {
    DBConfig::new(
        PORT,
        "questdb.default.svc.cluster.local".into(),
        EXCHANGE_TABLE_NAME.to_string(),
    )
}
