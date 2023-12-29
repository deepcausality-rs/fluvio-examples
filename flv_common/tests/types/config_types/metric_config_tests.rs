use common::prelude::MetricConfig;

#[test]
fn test_new() {
    let config = MetricConfig::new("metrics".to_string(), "localhost".to_string(), 8080);

    assert_eq!(config.metric_uri(), "metrics");
    assert_eq!(config.metric_host(), "localhost");
    assert_eq!(config.metric_port(), 8080);
}

#[test]
fn test_metric_uri() {
    let config = MetricConfig::new("custom".to_string(), "localhost".to_string(), 8081);

    assert_eq!(config.metric_uri(), "custom");
}

#[test]
fn test_metric_host() {
    let config = MetricConfig::new("metrics".to_string(), "metricshost".to_string(), 8082);

    assert_eq!(config.metric_host(), "metricshost");
}

#[test]
fn test_metric_port() {
    let config = MetricConfig::new("metrics".to_string(), "localhost".to_string(), 3000);

    assert_eq!(config.metric_port(), 3000);
}

