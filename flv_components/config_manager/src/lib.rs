use common::prelude::{DBConfig, EnvironmentType, ServiceConfig, ServiceID};

mod getters;
mod utils;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConfigManager {
    /// ID of this service.
    svc_id: ServiceID,
    /// Detected environment of this service.
    env_type: EnvironmentType,
    /// DB configuration relative to the detected environment.
    db_config: DBConfig,
    /// Configuration of this service.
    svc_config: ServiceConfig,
}

/// Construct a new ConfigManager instance.
///
/// This will detect the environment type, fetch the appropriate DB and
/// service configurations, and populate the fields.
///
/// # Arguments
///
/// * `svc_id` - The ID of the service to configure.
///
/// # Returns
///
/// A new ConfigManager instance with all fields populated based on the service ID.
impl ConfigManager {
    pub fn new(svc_id: ServiceID) -> Self {
        // Detect environment type
        let env_type = utils::get_env_type();

        // Get DB config for environment
        let db_config = utils::get_db_config(&env_type);

        // Get service config for ID
        let svc_config = utils::get_service_config(&svc_id);

        Self {
            svc_id,
            env_type,
            db_config,
            svc_config,
        }
    }
}
