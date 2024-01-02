use common::prelude::DBConfig;

#[test]
fn test_new() {
    let config = DBConfig::new(8000, "localhost".to_string());

    assert_eq!(config.port(), 8000);
    assert_eq!(config.host(), "localhost");
}

#[test]
fn test_debug() {
    let config = DBConfig::new(27017, "localhost".to_string());

    let expected = "DBConfig { port: 27017, host: \"localhost\" }";

    assert_eq!(format!("{:?}", config), expected);
}

#[test]
fn test_display() {
    let config = DBConfig::new(27017, "localhost".to_string());

    let expected = "DBConfig { port: 27017, host: localhost }";

    assert_eq!(format!("{}", config), expected);
}
