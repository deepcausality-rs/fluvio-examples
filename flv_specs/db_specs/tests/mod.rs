use db_specs::prelude::{get_cluster_db_config, get_local_db_config};

#[test]
fn test_get_local_db_config() {
    let local_config = get_local_db_config();
    // Assuming ClickHouseConfig::default() default values
    assert_eq!(local_config.url(), "127.0.0.1".to_string());
    assert_eq!(local_config.port(), 9000);
    assert_eq!(local_config.username(), "".to_string());
    assert_eq!(local_config.password(), "".to_string());
    assert_eq!(local_config.database(), "default".to_string());
}

#[test]
fn test_get_cluster_db_config() {
    let cluster_config = get_cluster_db_config();
    assert_eq!(
        cluster_config.url(),
        "http://clickhouse.default.svc.cluster.local".to_string()
    );
    assert_eq!(cluster_config.port(), 8123);
    assert_eq!(cluster_config.username(), "username".to_string());
    assert_eq!(cluster_config.password(), "password".to_string());
    assert_eq!(cluster_config.database(), "default".to_string());
}
