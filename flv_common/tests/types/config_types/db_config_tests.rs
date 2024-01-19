use common::prelude::DBConfig;

fn get_db_config() -> DBConfig {
    DBConfig::new(27017, "localhost".to_string())
}

#[test]
fn test_new() {
    let config = get_db_config();
    assert_eq!(config.port(), 27017);
    assert_eq!(config.host(), "localhost");
}

#[test]
fn test_pg_connection_string() {
    let config = get_db_config();

    // 8812 is the postgres default port.
    let expected = "user=admin password=quest host=localhost port=8812 dbname=qdb";

    assert_eq!(expected, config.pg_connection_string());
}

#[test]
fn test_debug() {
    let config = get_db_config();

    let expected = "DBConfig { port: 27017, host: \"localhost\", buffer_size: 50000 }";

    assert_eq!(format!("{:?}", config), expected);
}

#[test]
fn test_display() {
    let config = get_db_config();

    let expected = "DBConfig { port: 27017, host: localhost }";

    assert_eq!(format!("{}", config), expected);
}
