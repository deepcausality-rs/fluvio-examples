use common::prelude::{DBConfig, EnvironmentType, ExchangeID, ServiceConfig, ServiceID};
use db_specs::prelude::{get_cluster_db_config, get_local_db_config};
use service_specs::prelude::get_qdgw_service_config;
use std::collections::HashMap;
use std::env;
use std::path::Path;

const ERROR_MSG: &str = "Failed to detect ENV environment variable. Ensure ENV is set to either Local or Cluster or a local .env file is present.";

// Check if the environment variable is set to Local or Cluster.
// If so, return the environment type if the environment variable is known.
// If not, panic with an error message.
// If not, check if an .env file is present.
// If so, return local environment as the file only exists locally.
// If not, panic.
// Note, on Mac OS, shell environment variables are sanitized (erased) by default for security reasons
// thus the presence of an .env file at the project root is used to identify a local environment.
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
                "Unknown environment type: {}. Environment can only be Local or Cluster",
                env_var
            );
        }
    };
}

// Return the DB configuration for the detected environment type.
pub(crate) fn get_db_config(env_type: &EnvironmentType) -> DBConfig {
    match env_type {
        EnvironmentType::Local => get_local_db_config(),
        EnvironmentType::Cluster => get_cluster_db_config(),
    }
}

// returns the service configuration for the given service id.
pub(crate) fn get_service_config(id: &ServiceID) -> ServiceConfig {
    match id {
        ServiceID::QDGW => get_qdgw_service_config(),
        ServiceID::Default => ServiceConfig::default(),
    }
}

pub(crate) fn get_all_exchanges() -> Vec<ExchangeID> {
    exchange_specs::get_all_exchanges()
}

pub(crate) fn get_all_exchanges_ids_names() -> Vec<(u16, String)> {
    exchange_specs::get_all_exchanges_ids_names()
}

pub(crate) fn get_exchange_symbol_tables() -> HashMap<ExchangeID, String> {
    exchange_specs::get_exchange_symbol_tables()
}
