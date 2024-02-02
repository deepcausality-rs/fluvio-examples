use common::prelude::{MetricConfig, ServiceConfig, ServiceID};

/// Returns a ServiceConfig for the SYMDB service.
///
/// # Returns
///
/// ServiceConfig with:
/// - id: ServiceID::SYMDB
/// - name: "symdbv1"
/// - version: 1
/// - online: false
/// - description: "SYMDB gives access to symbol metadata"
/// - health_check_uri: "health"
/// - local_host: "0.0.0.0"
/// - local_port: [7070, 8081]
/// - cluster_host: "symdb-service.default.svc.cluster.local"
/// - cluster_port: [7070, 8081]
/// - dependencies: None
/// - metrics: MetricConfig with custom port 8081 to avoid port clashes on localhost.
///
/// # Remarks
///
/// Used to configure the SYMDB service.
///
pub fn get_symdb_service_config() -> ServiceConfig {
    let id = ServiceID::SYMDB;
    let name = "symdbv1".to_string();
    let version = 1;
    let online = false;
    let description =
        "SYMDB (Symbol Master Database) gives access to central symbol to ID mapping)".to_string();
    let health_check_uri = "health".to_string();
    let local_host = "0.0.0.0".to_string();
    let cluster_host = "symdb-service.default.svc.cluster.local".to_string();
    let local_port = get_symdb_ports();
    let cluster_port = get_symdb_ports();
    let dependencies = None;
    let metrics = get_symdb_metric_config();

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

fn get_symdb_ports() -> Vec<u16> {
    Vec::from([7070, 8081])
}

/// Returns a MetricConfig for the SYMDB service
/// with a custom metric_port to avoid port clashes on localhost.
///
/// # Returns
///
/// MetricConfig with:
/// - metric_uri: "metrics"
/// - metric_host: "0.0.0.0"
/// - metric_port: 8081 (alternative prometheus port)
///
/// # Remarks
///
/// Used to configure prometheus metrics for the SYMDB service.
///
fn get_symdb_metric_config() -> MetricConfig {
    let metric_host = "0.0.0.0".to_string();
    let metric_uri = "metrics".to_string();
    // Default port for prometheus metrics is 8080
    let metric_port = 8081;

    MetricConfig::new(metric_uri, metric_host, metric_port)
}
