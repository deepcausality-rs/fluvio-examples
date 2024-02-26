use common::prelude::{ClickHouseConfig, EnvironmentType, ExchangeID, ServiceConfig, ServiceID};
use db_specs::prelude::{get_cluster_db_config, get_local_db_config};
use service_specs::prelude::{get_qdgw_service_config, get_symdb_service_config};
use std::collections::HashMap;
use std::env;
use std::path::Path;

const ERROR_MSG: &str = "Failed to detect ENV environment variable. Ensure ENV is set to either Local or Cluster or a local .env file is present.";

/// Gets the environment type from the ENV variable or local .env file.
///
/// Tries to read the ENV variable. If missing, checks for a local .env file.
///
/// If ENV set or .env exists, returns EnvironmentType::Local.
///
/// If neither, panics with error message.
///
/// Converts ENV string value to EnvironmentType enum.
///
/// # Panics
///
/// Panics if both ENV and .env are missing.
///
/// # Returns
///
/// EnvironmentType::Local if ENV="Local" or .env exists.
/// EnvironmentType::Cluster if ENV="Cluster".
///
/// Note, on Mac OS, shell environment variables are sanitized (erased) by default for security reasons
/// thus the presence of an .env file at the project root is used to identify a local environment.
pub(crate) fn get_env_type() -> EnvironmentType {
    let env_var = match env::var("ENV") {
        Ok(val) => val,
        Err(_) => {
            let file_path = ".env";
            let path = Path::new(file_path);
            return if path.exists() {
                EnvironmentType::Local
            } else {
                panic!("{}", ERROR_MSG);
            };
        }
    };

    // Convert the environment variable to an EnvironmentType enum value.
    return match env_var.to_uppercase().as_str() {
        "LOCAL" => EnvironmentType::Local,
        "CLUSTER" => EnvironmentType::Cluster,
        _ => {
            panic!(
                "[ConfigManager]: Unknown environment type: {}. Environment can only be Local or Cluster",
                env_var
            );
        }
    };
}

/// Gets the database configuration for the given environment type.
///
/// Matches on the environment type to call the appropriate
/// DB config retrieval function.
///
/// # Arguments
///
/// * `env_type` - The EnvironmentType enum
///
/// # Returns
///
/// The DBConfig for the environment.
///
/// Specific configs:
///
/// - EnvironmentType::Local - Calls get_local_db_config()
/// - EnvironmentType::Cluster - Calls get_cluster_db_config()
///
pub(crate) fn get_db_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    match env_type {
        EnvironmentType::Local => get_local_db_config(),
        EnvironmentType::Cluster => get_cluster_db_config(),
    }
}

/// Gets the service configuration for the given service ID.
///
/// Matches on the service ID enum to call the appropriate
/// service config retrieval function.
///
/// # Arguments
///
/// * `id` - The ServiceID enum for the service
///
/// # Returns
///
/// The ServiceConfig for the given service ID.
///
/// Specific configs:
///
/// - ServiceID::QDGW - Calls get_qdgw_service_config()
/// - ServiceID::Default - Returns default config
pub(crate) fn get_service_config(id: &ServiceID) -> ServiceConfig {
    match id {
        ServiceID::QDGW => get_qdgw_service_config(),
        ServiceID::SYMDB => get_symdb_service_config(),
        ServiceID::Database => ServiceConfig::default(),
        ServiceID::Default => ServiceConfig::default(),
    }
}

/// Gets a vector containing all supported ExchangeID enums.
///
/// Calls the `get_all_exchanges()` function from the
/// `exchange_specs` crate to retrieve the list.
///
/// # Returns
///
/// A `Vec<ExchangeID>` containing all supported exchange IDs.
///
pub(crate) fn get_all_exchanges() -> Vec<ExchangeID> {
    exchange_specs::prelude::get_all_exchanges()
}

/// Gets a vector of ID and name pairs for all supported exchanges.
///
/// Calls the `get_all_exchanges_ids_names()` function from the
/// `exchange_specs` crate to retrieve the ID/name pairs.
///
/// The ID is a `u16` integer.
/// The name is a `String`.
///
/// # Returns
///
/// A `Vec<(u16, String)>` containing the ID and name pairs
/// for all supported exchanges.
///
pub(crate) fn get_all_exchanges_ids_names() -> Vec<(u16, String)> {
    exchange_specs::prelude::get_all_exchanges_ids_names()
}

/// Gets a hashmap containing symbol tables for all supported exchanges.
///
/// The symbol table for an exchange maps instrument symbols to instrument IDs.
///
/// Calls the `get_exchange_symbol_tables()` function from the
/// `exchange_specs` crate to retrieve the symbol tables.
///
/// # Returns
///
/// A `HashMap` mapping `ExchangeID` enums to symbol table `String`s for each exchange.
///
pub(crate) fn get_exchange_symbol_tables() -> HashMap<ExchangeID, String> {
    exchange_specs::prelude::get_exchange_symbol_tables()
}
