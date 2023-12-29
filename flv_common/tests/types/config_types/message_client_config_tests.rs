use common::prelude::MessageClientConfig;

#[test]
fn test_new() {
    let id = 100;
    let config = MessageClientConfig::new(id);

    assert_eq!(config.id(), id);
    assert_eq!(config.name(), "client-100");
}

#[test]
fn test_default() {
    let default = MessageClientConfig::default();

    assert_eq!(default.id(), 100);
    assert_eq!(default.name(), "default-client-100");
}

#[test]
fn test_channel_getters() {
    let id = 100;
    let config = MessageClientConfig::new(id);

    assert_eq!(config.control_channel(), "client-100-control");
    assert_eq!(config.data_channel(), "client-100-data");
    assert_eq!(config.execution_channel(), "client-100-execution");
}

#[test]
fn test_id() {
    let id = 100;
    let config = MessageClientConfig::new(id);

    assert_eq!(config.id(), id);
}

#[test]
fn test_name() {
    let id = 100;
    let config = MessageClientConfig::new(id);

    assert_eq!(config.name(), "client-100");
}

#[test]
fn test_display() {
    let id = 100;
    let config = MessageClientConfig::new(id);

    let actual = format!("{}", config);
    let expected = "MessageClientConfig { id: 100, name: client-100 }".to_string();

    assert_eq!(actual, expected);
}
