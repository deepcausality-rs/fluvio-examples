use common::prelude::{MetricConfig, ServiceConfig, ServiceID};

pub fn qdgw_service_config() -> ServiceConfig {
    let id = ServiceID::QDGW;
    let name = "qdgwv1".to_string();
    let version = 1;
    let online = false;
    let description = "QDGW gives access to quantitative tick data".to_string();
    let health_check_uri = "health".to_string();
    let local_host = "0.0.0.0".to_string();
    let local_port = Vec::from([9000, 9003, 9005, 9010]);
    let cluster_host = "qdgw-service.default.svc.cluster.local".to_string();
    let cluster_port = Vec::from([9000, 9003, 9005, 9010]);
    let dependencies = None;
    let metrics = get_metric_config();

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

fn get_metric_config() -> MetricConfig {
    let metric_host = "0.0.0.0".to_string();
    let metric_uri = "metrics".to_string();
    // Default port for prometheus metrics is 8080
    let metric_port = 8080;

    MetricConfig::new(metric_uri, metric_host, metric_port)
}
