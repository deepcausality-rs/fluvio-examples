#[cfg(test)]
mod tests {
    use common::prelude::ServiceID;
    use service_specs::prelude::qdgw_service_config;

    #[test]
  fn test_qdgw_service_config() {
    let config = qdgw_service_config();

    assert_eq!(config.svc_id(), ServiceID::QDGW);
    assert_eq!(config.name(), "qdgwv1");
    assert_eq!(config.version(), 1);
    assert_eq!(config.online(), false);
    assert_eq!(config.description(), "QDGW gives access to quantitative tick data");
    assert_eq!(config.health_check_uri(), "health");
    assert_eq!(config.local_host(), "0.0.0.0");
    assert_eq!(config.local_port(), vec![9000, 9003, 9005, 9010]);
    assert_eq!(config.cluster_host(), "qdgw-service.default.svc.cluster.local");
    assert_eq!(config.cluster_port(), vec![9000, 9003, 9005, 9010]);
    assert_eq!(config.dependencies(), &None);
    assert_eq!(config.metrics().metric_uri(), "metrics");
    assert_eq!(config.metrics().metric_host(), "0.0.0.0");
    assert_eq!(config.metrics().metric_port(), 8080);
  }

}
