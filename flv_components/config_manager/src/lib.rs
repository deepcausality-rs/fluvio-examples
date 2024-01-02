use common::prelude::{DBConfig, EnvironmentType, ServiceConfig, ServiceID};

mod utils;
mod getters;

const ERROR_MSG: &str = "Failed to detect ENV environment variable. Ensure ENV is set to either Local or Cluster or a local .env file is present.";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConfigManager {
    // ID of this service.
    svc_id: ServiceID,
    // Detected environment of this service.
    env_type: EnvironmentType,
    // DB configuration relative to the detected environment.
    db_config: DBConfig,
    // Configuration of this service.
    svc_config: ServiceConfig,
}

impl ConfigManager {
    pub fn new( svc_id: ServiceID) -> Self {
        let env_type = utils::get_env_type();
        let db_config = utils::get_db_config(&env_type);
        let svc_config = utils::get_service_config(&svc_id);

        Self {svc_id, env_type, db_config, svc_config }
    }
}
