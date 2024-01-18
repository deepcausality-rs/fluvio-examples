use common::prelude::MessageClientConfig;
use fluvio::{FluvioAdmin, TopicProducer};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use utils::flv_utils;

mod send_login;
mod send_logout;
mod send_start_data;
mod shared;
mod utils;

const TOPIC: &str = "qdgw-control";

/// The QDClient struct.
///
/// This holds the state for a QuantDesk client instance.
///
/// # Fields
///
/// * `client_id` - The unique ID for this client.
/// * `admin` - The FluvioAdmin client for administering topics.
/// * `client_config` - The client configuration.
/// * `producer` - The producer for sending messages to the gateway topic.
pub struct QDClient {
    client_id: u16,
    admin: FluvioAdmin,
    client_config: MessageClientConfig,
    producer: TopicProducer,
}

impl QDClient {
    /// Creates a new QDClient instance.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The unique ID for this client.
    /// * `consumer` - A closure that runs the message consumer.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the `QDClient` instance on success, or an `Error` on failure.
    /// The client will be already logged in to the gateway on success.
    ///
    /// This constructor does the following:
    ///
    /// - Creates a `MessageClientConfig` with the client ID.
    /// - Gets a `FluvioAdmin` client.
    /// - Gets a `TopicProducer` for the gateway topic.
    /// - Creates the client topics:
    ///   1. `client_id-control` - For receiving control messages from the gateway.///
    ///   2. `client_id-data` - For receiving data messages from the gateway.///
    ///   3. `client_id-error` - For receiving error messages from the gateway.
    /// - Spawns the `consumer` closure as a Tokio task.
    /// - Constructs the `QDClient`.
    /// - Logs in the client by calling `login()`.
    ///
    pub async fn new(
        client_id: u16,
        client_config: MessageClientConfig,
        consumer: Result<(), Box<dyn Error + Send>>,
    ) -> Result<Self, Box<dyn Error + Send>> {
        // Get Fluvio admin.
        let admin = flv_utils::get_admin()
            .await
            .expect("[QDClient/new]: Failed to get admin");

        // Get Fluvio producer for the gateway control topic.
        let producer = flv_utils::get_producer(TOPIC)
            .await
            .expect("[QDClient/new]: Failed to get producer");

        // Create client topics
        flv_utils::create_topics(&admin, &client_config)
            .await
            .expect("[QDClient/new]: Failed to create topics");

        // Start the consumer as Tokio task.
        tokio::spawn(async move {
            if let Err(e) = consumer {
                eprintln!("[QDClient/new]: Consumer connection error: {}", e);
            }
        });

        // Create client.
        let client = Self {
            client_id,
            admin,
            client_config,
            producer,
        };

        // login to the gateway.
        client
            .login()
            .await
            .expect("[QDClient/new]: Failed to log in");

        Ok(client)
    }
}

impl QDClient {
    /// Closes the client connection by logging out and deleting topics.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` on success, or an `Error` on failure.
    ///
    /// This logs the client out of the gateway and then deletes all client topics.
    ///
    pub async fn close(&self) -> Result<(), Box<dyn Error + Send>> {
        // Logs out of the gateway.
        self.logout()
            .await
            .expect("[QDClient/close]: Failed to log out");

        // Wait a moment for the logout to complete.
        sleep(Duration::from_millis(25)).await;

        // Delete client topics
        flv_utils::delete_topics(&self.admin, &self.client_config)
            .await
            .expect("[QDClient/close]: Failed to delete client topics");

        Ok(())
    }
}
