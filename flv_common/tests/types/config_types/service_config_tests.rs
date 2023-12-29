use common::prelude::{MetricConfig, ServiceConfig, ServiceID};

fn get_metric_config() -> MetricConfig {
    let metric_host = "0.0.0.0".to_string();
    let metric_uri = "metrics".to_string();
    // Default port for prometheus metrics is 8080
    let metric_port = 8080;

    MetricConfig::new(metric_uri, metric_host, metric_port)
}

#[test]
fn test_svc_id() {
    let config = ServiceConfig::new(ServiceID::QDGW, "My Service".to_string(), 1, true, "Desc".to_string(), "localhost".to_string(), vec![8080], "cluster".to_string(), vec![8081], "/health".to_string(), None,  get_metric_config() );
    assert_eq!(config.svc_id(), ServiceID::QDGW);
}


#[test]
fn test_name() {
    let config = ServiceConfig::new(ServiceID::QDGW, "My Service".to_string(), 1, true, "Desc".to_string(), "localhost".to_string(), vec![8080], "cluster".to_string(), vec![8081], "/health".to_string(), None,  get_metric_config() );
    assert_eq!(config.name(), "My Service");
}

#[test]
fn test_version() {
    let config = ServiceConfig::new(ServiceID::QDGW, "My Service".to_string(), 1, true, "Desc".to_string(), "localhost".to_string(), vec![8080], "cluster".to_string(), vec![8081], "/health".to_string(), None,  get_metric_config() );
    assert_eq!(config.version(), 1);
}

#[test]
fn test_online() {
    let config = ServiceConfig::new(ServiceID::QDGW, "My Service".to_string(), 1, true, "Desc".to_string(), "localhost".to_string(), vec![8080], "cluster".to_string(), vec![8081], "/health".to_string(), None,  get_metric_config() );
    assert_eq!(config.online(), true);
}