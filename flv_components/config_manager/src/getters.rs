use crate::ConfigManager;
use common::prelude::{
    DBConfig, EnvironmentType, MessageClientConfig, MetricConfig, ServiceConfig, ServiceID,
};

impl ConfigManager {
    pub fn get_svc_id(&self) -> ServiceID {
        self.svc_id
    }
    pub fn get_env_type(&self) -> EnvironmentType {
        self.env_type
    }
    pub fn get_db_config(&self) -> DBConfig {
        self.db_config.clone()
    }
    pub fn get_svc_config(&self) -> ServiceConfig {
        self.svc_config.clone()
    }
    pub fn get_message_client_config(&self) -> MessageClientConfig {
        MessageClientConfig::from_svc_id(self.svc_id)
    }
    pub fn get_svc_metric_config(&self) -> MetricConfig {
        self.svc_config.metrics().to_owned()
    }
}
