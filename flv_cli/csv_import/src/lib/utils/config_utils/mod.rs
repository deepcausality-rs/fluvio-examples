use crate::types::config_file::ConfigFile;
use common::prelude::DBConfig;
use std::error::Error;

/// Gets the configuration file from the given path.
///
/// Tries to load the ConfigFile from the path.
///
/// Returns Ok(ConfigFile) if successful.
/// Returns Err(e) if failed to load file.
///
/// # Arguments
///
/// * `path` - The path to the config file
///
/// # Returns
///
/// * `Ok(ConfigFile)` - The loaded config file
/// * `Err(e)` - Error if failed to load file
///
pub fn get_config_file(path: &str) -> Result<ConfigFile, Box<dyn Error>> {
    match ConfigFile::from_file(path) {
        Ok(config) => Ok(config),
        Err(e) => Err(e),
    }
}

/// Gets the local database configuration.
///
/// Calls the `get_local_db_config()` function from the
/// `db_specs` crate to retrieve the local DBConfig.
///
/// # Returns
///
/// The `DBConfig` for connecting to the local database.
///
pub fn get_local_db_config() -> DBConfig {
    db_specs::prelude::get_local_db_config()
}
