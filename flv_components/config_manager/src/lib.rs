use common::prelude::{DBConfig, EnvironmentType, ExchangeID, ServiceConfig, ServiceID};
use std::collections::HashMap;

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
    /// Default exchange
    default_exchange: ExchangeID,
    /// Vector of all supported exchanges.
    exchanges: Vec<ExchangeID>,
    /// Maps exchange IDs to their names. Used to configure Symbol Manager
    exchanges_id_names: Vec<(u16, String)>,
    /// Maps exchange IDs to their symbol table. Used to configure Query Manager
    exchanges_symbol_tables: HashMap<ExchangeID, String>,
}

impl ConfigManager {
    /// Create a new ConfigManager instance.
    ///
    /// This will:
    ///
    /// - Detect the environment type
    /// - Get the DB config for the environment
    /// - Get the service config for the provided service ID
    /// - Set default exchange to Kraken
    /// - Get all supported exchanges
    /// - Get exchange ID/name pairs
    /// - Get symbol tables for exchanges
    ///
    /// # Parameters
    ///
    /// * `svc_id` - The ServiceID for this service
    ///
    /// # Returns
    ///
    /// A ConfigManager instance initialized with all configuration.
    ///
    pub fn new(svc_id: ServiceID) -> Self {
        // Detect environment type
        let env_type = utils::get_env_type();

        // Get DB config for environment
        let db_config = utils::get_db_config(&env_type);

        // Get service config for ID
        let svc_config = utils::get_service_config(&svc_id);

        // Get default exchange for service
        let default_exchange = ExchangeID::Kraken;

        // Get vector of all supported exchanges
        let exchanges = utils::get_all_exchanges();

        let exchanges_id_names = utils::get_all_exchanges_ids_names();

        // Get hashmap of symbol tables for all supported exchanges
        let exchanges_symbol_tables = utils::get_exchange_symbol_tables();

        Self {
            svc_id,
            env_type,
            db_config,
            svc_config,
            default_exchange,
            exchanges,
            exchanges_id_names,
            exchanges_symbol_tables,
        }
    }
}
