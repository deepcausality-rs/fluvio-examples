use common::prelude::{MetricConfig, ServiceConfig, ServiceID};

/// Returns a ServiceConfig for the QDGW service.
///
/// # Returns
///
/// ServiceConfig with:
/// - id: ServiceID::QDGW
/// - name: "qdgwv1"
/// - version: 1
/// - online: false
/// - description: "QDGW gives access to quantitative tick data"
/// - health_check_uri: "health"
/// - local_host: "0.0.0.0"
/// - local_port: [9000, 9003, 9005, 9010, 8080]
/// - cluster_host: "qdgw-service.default.svc.cluster.local"
/// - cluster_port: [9000, 9003, 9005, 9010, 8080]
/// - dependencies: None
/// - metrics:  MetricConfig with:
/// - metric_uri: "metrics"
/// - metric_host: "0.0.0.0"
/// - metric_port: 8080 (default prometheus port)
///
/// # Remarks
///
/// Used to configure the QDGW service.
///
pub fn get_qdgw_service_config() -> ServiceConfig {
    let id = ServiceID::QDGW;
    let name = "qdgwv1".to_string();
    let version = 1;
    let online = false;
    let description =
        "QDGW QDGW (Quantitative Data Gateway) gives access to quantitative tick data".to_string();
    let health_check_uri = "health".to_string();
    let local_host = "0.0.0.0".to_string();
    let cluster_host = "qdgw-service.default.svc.cluster.local".to_string();
    let local_port = get_qdgw_ports();
    let cluster_port = get_qdgw_ports();
    let dependencies = None;
    let metrics = get_qdgw_metric_config();

    ServiceConfig::new(
        id,
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
    )
}

/// Returns a vector of u16 containing the ports used by the QDGW service.
///
/// The ports returned are:
///
/// - 9000: Fluvio Streaming port
/// - 9003: Fluvio Streaming port
/// - 9005: Fluvio Streaming port
/// - 9010: Metrics port
/// - 8080: Prometheus metrics port (default)
///
/// # Returns
///
/// A vector of u16 containing the ports: [9000, 9003, 9005, 9010, 8080]
///
fn get_qdgw_ports() -> Vec<u16> {
    Vec::from([9000, 9003, 9005, 9010, 8080])
}

/// Returns a MetricConfig for the QDGW service.
///
/// # Returns
///
/// MetricConfig with:
/// - metric_uri: "metrics"
/// - metric_host: "0.0.0.0"
/// - metric_port: 8080 (default prometheus port)
///
/// # Remarks
///
/// Used to configure prometheus metrics for the QDGW service.
///
fn get_qdgw_metric_config() -> MetricConfig {
    let metric_host = "0.0.0.0".to_string();
    let metric_uri = "metrics".to_string();
    // Default port for prometheus metrics is 8080
    let metric_port = 8080;

    MetricConfig::new(metric_uri, metric_host, metric_port)
}
