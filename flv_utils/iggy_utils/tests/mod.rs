use common::prelude::{IggyConfig, IggyUser};
use iggy_utils::{cleanup, get_consumer, get_iggy_client, get_producer, shutdown};

#[tokio::test]
async fn test_get_iggy_client() {
    let tcp_server_addr = "127.0.0.1:3000".to_string();

    let result = get_iggy_client(tcp_server_addr).await;
    assert!(result.is_ok());

    let client = result.unwrap();
    shutdown(&client).await.expect("Failed to shutdown client");

    let error_client = get_iggy_client("invalid".to_string()).await;

    assert!(error_client.is_err());
}

#[tokio::test]
async fn test_get_consumer() {
    // Arrange
    let user = IggyUser::default();
    let iggy_config = IggyConfig::from_client_id(user, 5, 50, true);

    // Act
    let result = get_consumer(&iggy_config).await;

    // Assert
    assert!(result.is_ok());
    let client = result.unwrap();
    shutdown(&client).await.expect("Failed to shutdown client");
}

#[tokio::test]
async fn test_get_consumer_init_error() {
    // Arrange
    let user = IggyUser::new("invalid", "invalid");
    let iggy_config = IggyConfig::from_client_id(user, 5, 50, true);

    // Act
    let result = get_consumer(&iggy_config).await;

    // Assert
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_producer() {
    // Arrange
    let user = IggyUser::default();
    let iggy_config = IggyConfig::from_client_id(user, 5, 50, true);

    // Act
    let result = get_producer(&iggy_config).await;

    // Assert
    assert!(result.is_ok());

    // Cleanup
    let client = result.unwrap();
    cleanup(&client, &iggy_config)
        .await
        .expect("Failed to cleanup client");
    shutdown(&client).await.expect("Failed to shutdown client");
}

#[tokio::test]
async fn test_get_producer_init_error() {
    // Arrange
    let user = IggyUser::new("invalid", "invalid");
    let config = IggyConfig::from_client_id(user, 5, 50, true);

    // Act
    let result = get_producer(&config).await;

    // Assert
    assert!(result.is_err());
}
