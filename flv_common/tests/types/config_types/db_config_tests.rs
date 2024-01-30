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
fn test_new_with_pg_config() {
    let config = DBConfig::new_with_pg_config(
        27017,
        "localhost".to_string(),
        "pguser".to_string(),
        "pgpass".to_string(),
        "pgdb".to_string(),
        5432,
        10,
    );

    assert_eq!(config.port(), 27017);
    assert_eq!(config.host(), "localhost");
    assert_eq!(config.pg_user(), "pguser");
    assert_eq!(config.pg_password(), "pgpass");
    assert_eq!(config.pg_database(), "pgdb");
    assert_eq!(config.pg_port(), 5432);
}

#[test]
fn test_pg_connection_string() {
    let config = get_db_config();

    // 8812 is the postgres default port.
    let expected = "user=admin password=quest host=localhost port=8812 dbname=qdb";

    assert_eq!(expected, config.pg_connection_string());
}

#[test]
fn test_display() {
    let config = DBConfig::new_with_pg_config(
        27017,
        "localhost".to_string(),
        "pguser".to_string(),
        "pgpass".to_string(),
        "pgdb".to_string(),
        5432,
        10,
    );

    let expected = "DBConfig {\n  port: 27017,\n  host: localhost,\n  pg_user: pguser,\n  pg_database: pgdb\n pg_port: 5432\n}";

    assert_eq!(format!("{}", config), expected);
}
