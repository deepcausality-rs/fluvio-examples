use common::prelude::DBConfig;

#[test]
fn test_new() {
    let config = DBConfig::new(8000, "localhost".to_string(), "exchanges".to_string());

    assert_eq!(config.port(), 8000);
    assert_eq!(config.host(), "localhost");
}

#[test]
fn test_pg_connection_string() {
    let config = DBConfig::new(27017, "localhost".to_string(), "exchanges".to_string());

    let expected = "user=admin password=quest host=localhost port=8812 dbname=qdb";

    assert_eq!(expected, config.pg_connection_string());
}

#[test]
fn test_debug() {
    let config = DBConfig::new(27017, "localhost".to_string(), "exchanges".to_string());

    let expected = "DBConfig { port: 27017, host: \"localhost\", exchange_table_name: \"exchanges\", buffer_size: 50000 }";

    assert_eq!(format!("{:?}", config), expected);
}

#[test]
fn test_display() {
    let config = DBConfig::new(27017, "localhost".to_string(), "exchanges".to_string());

    let expected = "DBConfig { port: 27017, host: localhost, exchange_table_name: exchanges }";

    assert_eq!(format!("{}", config), expected);
}
