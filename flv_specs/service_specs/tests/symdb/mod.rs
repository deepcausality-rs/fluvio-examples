use common::prelude::ServiceID;
use service_specs::prelude::get_symdb_service_config;

#[test]
fn test_get_symdb_service_config() {
    let config = get_symdb_service_config();

    assert_eq!(config.svc_id(), ServiceID::SYMDB);
    assert_eq!(config.name(), "symdbv1");
    assert_eq!(config.version(), 1);
    assert_eq!(config.online(), false);
    assert_eq!(
        config.description(),
        "SYMDB (Symbol Master Database) gives access to central symbol to ID mapping)"
    );
    assert_eq!(config.health_check_uri(), "health");

    assert_eq!(config.local_host(), "0.0.0.0");
    assert_eq!(config.local_port(), vec![7070, 8081]);

    assert_eq!(
        config.cluster_host(),
        "symdb-service.default.svc.cluster.local"
    );
    assert_eq!(config.cluster_port(), vec![7070, 8081]);

    assert!(config.dependencies().is_none());
    assert_eq!(config.metrics().metric_uri(), "metrics");
    assert_eq!(config.metrics().metric_host(), "0.0.0.0");
    assert_eq!(config.metrics().metric_port(), 8081);
}
