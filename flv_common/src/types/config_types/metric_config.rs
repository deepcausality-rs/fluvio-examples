use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MetricConfig {
    metric_uri: String,
    metric_host: String,
    metric_port: u16,
}

impl MetricConfig {
    pub fn new(metric_uri: String, metric_host: String, metric_port: u16) -> Self {
        Self {
            metric_uri,
            metric_host,
            metric_port,
        }
    }
}

impl Default for MetricConfig {
    fn default() -> Self {
        let metric_host = "0.0.0.0".to_string();
        let metric_uri = "metrics".to_string();
        let metric_port = 8080; // Default port for prometheus is 8080

        MetricConfig::new(metric_uri, metric_host, metric_port)
    }
}

impl MetricConfig {
    pub fn metric_uri(&self) -> &str {
        &self.metric_uri
    }
    pub fn metric_host(&self) -> &str {
        &self.metric_host
    }
    pub fn metric_port(&self) -> u16 {
        self.metric_port
    }
}

impl Display for MetricConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "metric_uri: {},  metric_host: {},  metric_port: {}",
            self.metric_uri, self.metric_host, self.metric_port
        )
    }
}
