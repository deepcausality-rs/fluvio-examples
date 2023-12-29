use common::prelude::DBConfig;

#[test]
fn test_new() {
    let config = DBConfig::new(
        8000,
        "localhost".to_string(),
        "test".to_string(),
        "test".to_string(),
        "username".to_string(),
        "password".to_string(),
        "client_name".to_string(),
    );

    assert_eq!(config.port(), 8000);
    assert_eq!(config.host(), "localhost");
    assert_eq!(config.db_name(), "test");
    assert_eq!(config.db_namespace(), "test");
    assert_eq!(config.username(), "username");
    assert_eq!(config.password(), "password");
    assert_eq!(config.client_name(), "client_name");
}

#[test]
fn test_new_connection() {
    let config = DBConfig::new_connection(8000, "localhost".to_string());

    assert_eq!(config.port(), 8000);
    assert_eq!(config.host(), &("localhost".to_string()));
}

#[test]
fn test_new_connection_with_authentication() {
    let config = DBConfig::new_connection_with_authentication(
        8000,
        "localhost".to_string(),
        "username".to_string(),
        "password".to_string(),
    );

    assert_eq!(config.port(), 8000);
    assert_eq!(config.host(), &("localhost".to_string()));
    assert_eq!(config.username(), &("username".to_string()));
    assert_eq!(config.password(), &("password".to_string()));
}

#[test]
fn test_new_authentication() {
    let config = DBConfig::new_authentication("username".to_string(), "password".to_string());

    assert_eq!(config.username(), &("username".to_string()));
    assert_eq!(config.password(), &("password".to_string()));
}

#[test]
fn test_default() {
    let config = DBConfig::default();

    assert_eq!(config.port(), 8000);
    assert_eq!(config.host(), &"0.0.0.0".to_string());
    assert_eq!(config.username(), &"root".to_string());
    assert_eq!(config.password(), &"root".to_string());
    assert_eq!(config.client_name(), &"dbgw".to_string());
}

#[test]
fn test_debug() {
    let config = DBConfig::new_connection_with_authentication(
        8000,
        "localhost".to_string(),
        "username".to_string(),
        "password".to_string(),
    );

    let expected = "DBConfig { port: 8000, host: \"localhost\", db_name: \"test\", db_namespace: \"test\", username: \"username\", password: \"password\", client_name: \"dbgw\" }";
    let actual = format!("{:?}", config);
    assert_eq!(expected, actual);
}

#[test]
fn test_display() {
    let config = DBConfig::new_connection_with_authentication(
        8000,
        "localhost".to_string(),
        "username".to_string(),
        "password".to_string(),
    );

    let expected = "DBConfig { port: 8000, host: localhost, db_name: test, db_namespace: test, username: username,password: password, client_name: dbgw }";
    let actual = config.to_string();
    assert_eq!(expected, actual);
}
