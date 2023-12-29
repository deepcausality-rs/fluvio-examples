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
    );

    assert_eq!(config.port(), 8000);
    assert_eq!(config.host(), "localhost");
    assert_eq!(config.db_name(), "test");
    assert_eq!(config.db_namespace(), "test");
    assert_eq!(config.username(), "username");
    assert_eq!(config.password(), "password");
}



  #[test]
  fn test_debug() {
    let config = DBConfig::new(
      27017,
      "localhost".to_string(),
      "mydb".to_string(),
      "test".to_string(),
      "admin".to_string(),
      "password".to_string()
    );

    let expected = "DBConfig { port: 27017, host: \"localhost\", db_name: \"mydb\", db_namespace: \"test\", username: \"admin\", password: \"password\" }";

    assert_eq!(format!("{:?}", config), expected);
  }

  #[test]
  fn test_display() {
    let config = DBConfig::new(
      27017,
      "localhost".to_string(),
      "mydb".to_string(),
      "test".to_string(),
      "admin".to_string(),
      "password".to_string()
    );

    let expected = "DBConfig { port: 27017, host: localhost, db_name: mydb, db_namespace: test, username: admin,password: password }";

    assert_eq!(format!("{}", config), expected);
  }

