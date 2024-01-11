use crate::types::config_file::ConfigFile;
use common::prelude::DBConfig;
use std::error::Error;

pub fn get_config_file(path: &str) -> Result<ConfigFile, Box<dyn Error>> {
    match ConfigFile::from_file(path) {
        Ok(config) => Ok(config),
        Err(e) => Err(e),
    }
}

pub fn get_local_db_config() -> DBConfig {
    db_specs::prelude::get_local_db_config()
}
