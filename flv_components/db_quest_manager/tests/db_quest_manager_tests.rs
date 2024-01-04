use common::prelude::DBConfig;

const PATH: &str = "data/Kraken_Trading_History/1INCHUSD.csv";

fn get_local_db_config() -> DBConfig {
    DBConfig::new(9009, "0.0.0.0".into())
}
