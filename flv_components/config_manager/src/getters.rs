use crate::ConfigManager;
use common::prelude::{
    DBConfig, EnvironmentType, MessageClientConfig, MetricConfig, ServiceConfig, ServiceID,
};

impl ConfigManager {
    /// Get the service ID associated with this ConfigManager.
    ///
    /// # Returns
    ///
    /// The ServiceID for this config.
    pub fn get_svc_id(&self) -> ServiceID {
        self.svc_id
    }

    /// Get the environment type for this ConfigManager.
    ///
    /// # Returns
    ///
    /// The EnvironmentType enum variant.
    pub fn get_env_type(&self) -> EnvironmentType {
        self.env_type
    }

    /// Get a copy of the DBConfig associated with this service.
    ///
    /// # Returns
    ///
    /// A cloned DBConfig value.
    pub fn get_db_config(&self) -> DBConfig {
        self.db_config.clone()
    }

    /// Get a copy of the ServiceConfig associated with this service.
    ///
    /// # Returns
    ///
    /// A cloned ServiceConfig value.
    pub fn get_svc_config(&self) -> ServiceConfig {
        self.svc_config.clone()
    }

    /// Get the MessageClientConfig for this service.
    ///
    /// # Returns
    ///
    /// The MessageClientConfig for the service.
    pub fn get_message_client_config(&self) -> MessageClientConfig {
        MessageClientConfig::from_svc_id(self.svc_id)
    }

    /// Get the MetricConfig from this service.
    ///
    /// # Returns
    ///
    /// The MetricConfig for the service.
    pub fn get_svc_metric_config(&self) -> MetricConfig {
        self.svc_config.metrics().to_owned()
    }
}
