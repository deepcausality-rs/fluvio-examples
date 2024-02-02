use crate::ConfigManager;
use common::prelude::{
    DBConfig, EnvironmentType, ExchangeID, MessageClientConfig, MetricConfig, ServiceConfig,
    ServiceID,
};

const DEFAULT_HOST: &str = "0.0.0.0";

impl ConfigManager {
    /// Get the default ExchangeID configured for this service.
    ///
    /// # Returns
    ///
    /// The `default_exchange` field, containing the u16 ExchangeID value
    /// set as the default for this service.
    pub fn get_symbol_table(&self, exchange_id: ExchangeID) -> Option<String> {
        self.exchanges_symbol_tables.get(&exchange_id).cloned()
    }
}

impl ConfigManager {
    pub fn get_svc_socket_addr(&self) -> String {
        let host = DEFAULT_HOST;
        let local_ports = self.svc_config.local_port();
        let port = local_ports.get(1).unwrap();

        format!("{}:{}", host, port)
    }
}

impl ConfigManager {
    /// Get the service ID associated with this ConfigManager.
    ///
    /// # Returns
    ///
    /// The ServiceID for this config.
    pub fn svc_id(&self) -> ServiceID {
        self.svc_id
    }

    /// Get the environment type for this ConfigManager.
    ///
    /// # Returns
    ///
    /// The EnvironmentType enum variant.
    pub fn env_type(&self) -> EnvironmentType {
        self.env_type
    }

    /// Get a copy of the DBConfig associated with this service.
    ///
    /// # Returns
    ///
    /// A cloned DBConfig value.
    pub fn db_config(&self) -> DBConfig {
        self.db_config.clone()
    }

    /// Get a copy of the ServiceConfig associated with this service.
    ///
    /// # Returns
    ///
    /// A cloned ServiceConfig value.
    pub fn svc_config(&self) -> ServiceConfig {
        self.svc_config.clone()
    }

    /// Get the MessageClientConfig for this service.
    ///
    /// # Returns
    ///
    /// The MessageClientConfig for the service.
    pub fn message_client_config(&self) -> MessageClientConfig {
        MessageClientConfig::from_svc_id(self.svc_id)
    }

    /// Get the MetricConfig from this service.
    ///
    /// # Returns
    ///
    /// The MetricConfig for the service.
    pub fn svc_metric_config(&self) -> MetricConfig {
        self.svc_config.metrics().to_owned()
    }

    /// Get a reference to the vector of configured ExchangeIDs.
    ///
    /// # Returns
    ///
    /// A reference to the `exchanges` field, which is a vector containing
    /// the ExchangeID values configured for this service.
    pub fn exchanges(&self) -> &Vec<ExchangeID> {
        &self.exchanges
    }

    /// Get a reference to the vector containing ExchangeID and name pairs.
    ///
    /// # Returns
    ///
    /// A reference to the `exchanges_id_names` field, which contains a vector
    /// of tuples with the first element being the ExchangeID u16 and the
    /// second element being the corresponding exchange name string.
    pub fn exchanges_id_names(&self) -> &Vec<(u16, String)> {
        &self.exchanges_id_names
    }

    /// Get the default ExchangeID configured for this service.
    ///
    /// # Returns
    ///
    /// The `default_exchange` field, containing the u16 ExchangeID value
    /// set as the default for this service.
    pub fn default_exchange(&self) -> ExchangeID {
        self.default_exchange
    }
}
