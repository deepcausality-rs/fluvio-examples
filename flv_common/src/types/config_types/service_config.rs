use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};
// use surrealdb::sql::Thing;

use crate::prelude::{MetricConfig, ServiceID};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct ServiceConfig {
    // // DB ID
    // id: Option<Thing>,
    /// Unique Service ID.
    svc_id: ServiceID,
    /// Service name.
    name: String,
    /// Service version.
    version: u8,
    /// Whether the service is online.
    online: bool,
    /// Service description.
    description: String,
    /// local host.
    local_host: String,
    /// local port.
    local_port: Vec<u16>,
    /// Cluster host.
    cluster_host: String,
    /// Cluster port.
    cluster_port: Vec<u16>,
    /// Health check URI relative to local or cluster host.
    health_check_uri: String,
    /// Service dependencies.
    dependencies: Option<Vec<ServiceID>>,
    /// Service metrics configuration for Prometheus.
    metrics: MetricConfig,
}

impl ServiceConfig {
    pub fn new(
        svc_id: ServiceID,
        name: String,
        version: u8,
        online: bool,
        description: String,
        local_host: String,
        local_port: Vec<u16>,
        cluster_host: String,
        cluster_port: Vec<u16>,
        health_check_uri: String,
        dependencies: Option<Vec<ServiceID>>,
        metrics: MetricConfig,
    ) -> Self {
        Self {
            svc_id,
            name,
            version,
            online,
            description,
            local_host,
            local_port,
            cluster_host,
            cluster_port,
            health_check_uri,
            dependencies,
            metrics,
        }
    }
}

impl ServiceConfig {
    pub fn svc_id(&self) -> ServiceID {
        self.svc_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> u8 {
        self.version
    }
    pub fn online(&self) -> bool {
        self.online
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn local_host(&self) -> &str {
        &self.local_host
    }
    pub fn local_port(&self) -> Vec<u16> {
        self.local_port.to_owned()
    }
    pub fn cluster_host(&self) -> &str {
        &self.cluster_host
    }
    pub fn cluster_port(&self) -> Vec<u16> {
        self.cluster_port.to_owned()
    }
    pub fn health_check_uri(&self) -> &str {
        &self.health_check_uri
    }
    pub fn dependencies(&self) -> &Option<Vec<ServiceID>> {
        &self.dependencies
    }
    pub fn metrics(&self) -> &MetricConfig {
        &self.metrics
    }
}

impl Display for ServiceConfig {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "ServiceConfig[id: {}, name: {}, version: {}, online: {}, description: {}, local_host: {}, local_port: {:?}, cluster_host: {}, cluster_port: {:?}, health_check_uri: {}, dependencies: {:?}, metrics: {}]",
           self.svc_id(),
           self.name(),
           self.version(),
           self.online(),
           self.description(),
           self.local_host(),
           self.local_port(),
           self.cluster_host(),
           self.cluster_port(),
           self.health_check_uri(),
           self.dependencies(),
           self.metrics())
    }
}
