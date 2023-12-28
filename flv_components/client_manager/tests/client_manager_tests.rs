use client_manager::ClientManager;
use common::prelude::MessageClientConfig;

#[test]
fn test_add_client() {
    let mut manager = ClientManager::new();

    let id = 100;
    let config = MessageClientConfig::new(id);
    manager
        .add_client(id, config)
        .expect("Failed to add client");

    assert!(manager.check_client(id));
}

#[test]
fn test_get_client_control_channel() {
    let mut manager = ClientManager::new();
    let id = 100;
    let config = MessageClientConfig::new(id);
    manager
        .add_client(id, config.clone())
        .expect("Failed to add client");

    let result = manager.get_client_control_channel(id);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), config.clone().control_channel());

    let result = manager.get_client_control_channel(2);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "MessageClientConfigError: Client id 2 does not exist"
    );
}

#[test]
fn test_get_client_data_channel() {
    let mut manager = ClientManager::new();
    let id = 100;
    let config = MessageClientConfig::new(id);
    manager
        .add_client(id, config.clone())
        .expect("Failed to add client");

    let result = manager.get_client_data_channel(id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), config.clone().data_channel());

    let result = manager.get_client_data_channel(2);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "MessageClientConfigError: Client id 2 does not exist"
    );
}

#[test]
fn test_get_client_execution_channel() {
    let mut manager = ClientManager::new();
    let id = 100;
    let config = MessageClientConfig::new(id);
    manager
        .add_client(id, config.clone())
        .expect("Failed to add client");
    let result = manager.get_client_execution_channel(id);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), config.execution_channel());

    let result = manager.get_client_execution_channel(2);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "MessageClientConfigError: Client id 2 does not exist"
    );
}

#[test]
fn test_get_client() {
    let mut manager = ClientManager::new();
    let id = 100;
    let config = MessageClientConfig::new(id);
    manager
        .add_client(id, config)
        .expect("Failed to add client");

    let res = manager.get_client_config(id);
    assert!(res.is_ok());
}

#[test]
fn test_remove_client() {
    let mut manager = ClientManager::new();
    let id = 100;
    let config = MessageClientConfig::new(id);
    manager
        .add_client(id, config)
        .expect("Failed to add client");

    assert!(manager.check_client(id));

    manager.remove_client(id);

    assert!(!manager.check_client(id));
}
