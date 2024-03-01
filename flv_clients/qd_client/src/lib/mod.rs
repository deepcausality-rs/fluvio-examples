use common::prelude::MessageClientConfig;
use fluvio::FluvioAdmin;
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
///
pub struct QDClient {
    client_id: u16,
    admin: FluvioAdmin,
    client_config: MessageClientConfig,
    producer: fluvio::TopicProducer,
}

impl QDClient {
    /// Creates a new QDClient instance.
    pub async fn new(
        client_id: u16,
        client_config: MessageClientConfig,
    ) -> Result<Self, Box<dyn Error + Send>> {
        // Get Fluvio admin.
        let admin = flv_utils::get_admin()
            .await
            .expect("[QDClient/new]: Failed to get admin");

        // Get a producer for the gateway control topic.
        let producer = flv_utils::get_producer(TOPIC)
            .await
            .expect("[QDClient/new]: Failed to get producer for gateway topic");

        // Create client topics
        flv_utils::create_topics(&admin, &client_config)
            .await
            .expect("[QDClient/new]: Failed to create topics");

        // Create client.
        let client = Self {
            client_id,
            admin,
            client_config,
            producer,
        };

        // login to the QD gateway and register the clients data channel
        // to receive data streamed from the gateway.
        client
            .login()
            .await
            .expect("[QDClient/new]: Failed to log in to the QD Gateway");

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
