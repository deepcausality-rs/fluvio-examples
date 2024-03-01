use common::prelude::{
    ClickHouseConfig, EnvironmentType, ExchangeID, IggyConfig, ServiceConfig, ServiceID,
};
use std::collections::HashMap;

mod getters;
mod utils;

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigManager {
    /// ID of this service.
    svc_id: ServiceID,
    /// Detected environment of this service.
    env_type: EnvironmentType,
    /// DB configuration relative to the detected environment.
    db_config: ClickHouseConfig,
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
    //
    iggy_config: IggyConfig,
}

impl ConfigManager {
    /// Creates a new ConfigManager instance.
    ///
    /// Detects the environment type, gets the DB and service configs,
    /// default exchange, supported exchanges, exchange ID/name pairs,
    /// and exchange symbol tables.
    ///
    /// Populates a new ConfigManager with this data.
    ///
    /// # Arguments
    ///
    /// * `svc_id` - The service ID for this instance
    ///
    /// # Returns
    ///
    /// A ConfigManager instance configured with:
    ///
    /// - The detected environment
    /// - DB config for the environment
    /// - Service config for the ID
    /// - Default exchange
    /// - Supported exchanges
    /// - Exchange ID/name pairs
    /// - Exchange symbol tables
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::env;
    /// use common::prelude::ServiceID;
    /// use config_manager::ConfigManager;
    ///     // Set environment variable for testing purposes
    ///     // Usually this would be read from the environment or the .env file.
    ///     env::set_var("ENV", "Local");
    ///
    ///     let svc_id = ServiceID::Default;
    ///     let config_manager = ConfigManager::new(svc_id);
    ///     assert_eq!(svc_id, config_manager.svc_id());
    /// ```
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

        // Get vector of all exchange ID/name pairs
        let exchanges_id_names = utils::get_all_exchanges_ids_names();

        // Get hashmap of symbol tables for all supported exchanges
        let exchanges_symbol_tables = utils::get_exchange_symbol_tables();

        let iggy_config = utils::get_iggy_config(&env_type, svc_id);

        Self {
            svc_id,
            env_type,
            db_config,
            svc_config,
            default_exchange,
            exchanges,
            exchanges_id_names,
            exchanges_symbol_tables,
            iggy_config,
        }
    }
}
