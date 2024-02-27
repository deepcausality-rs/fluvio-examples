use common::prelude::ClickHouseConfig;

#[test]
fn test_click_house_config_new() {
    let config = ClickHouseConfig::new(
        "http://example.com".to_string(),
        1234,
        "user".to_string(),
        "pass".to_string(),
        "db".to_string(),
    );
    assert_eq!(config.url(), "http://example.com");
    assert_eq!(config.port(), 1234);
    assert_eq!(config.username(), "user");
    assert_eq!(config.password(), "pass");
    assert_eq!(config.database(), "db");
}

#[test]
fn test_click_house_config_default() {
    let config = ClickHouseConfig::default();
    assert_eq!(config.url(), "127.0.0.1");
    assert_eq!(config.port(), 9000);
    assert_eq!(config.username(), "");
    assert_eq!(config.password(), "");
    assert_eq!(config.database(), "default");
}

#[test]
fn test_click_house_config_connection_string() {
    let config = ClickHouseConfig::default();
    assert_eq!(config.connection_string(), "127.0.0.1:9000");
}

#[test]
fn test_click_house_config_accessors() {
    let config = ClickHouseConfig::default();
    assert_eq!(config.url(), "127.0.0.1");
    assert_eq!(config.port(), 9000);
    assert_eq!(config.username(), "");
    assert_eq!(config.password(), "");
    assert_eq!(config.database(), "default");
}

#[test]
fn test_click_house_config_display() {
    let config = ClickHouseConfig::default();

    let expected = "ClickHouseConfig { url: 127.0.0.1, port: 9000, database: default, username:  }";
    let actual = format!("{}", config);
    assert_eq!(expected, actual);
}
