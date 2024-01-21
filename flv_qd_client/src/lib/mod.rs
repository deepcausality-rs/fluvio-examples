use common::prelude::MessageClientConfig;
use fluvio::{FluvioAdmin, Offset, TopicProducer};
use futures::StreamExt;
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
    producer: TopicProducer,
}

impl QDClient {
    /// Creates a new QDClient instance.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The unique ID for this client.
    /// * `client_config` - The client configuration.
    /// * `data_handler` - The event handler for data messages.
    /// * `err_handler` - The event handler for error messages.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the `QDClient` instance on success, or an `Error` on failure.
    ///
    /// # Note, the constructor does the following:
    ///
    /// - Gets a `FluvioAdmin` client and `TopicProducer` for the gateway.
    /// - Creates the client topics using `flv_utils::create_topics()`.
    /// - Spawns the `data_handler` and `err_handler` callbacks as tasks.
    /// - Constructs the `QDClient`.
    /// - Logs in the client by calling `login()`.
    ///
    pub async fn new(
        client_id: u16,
        client_config: MessageClientConfig,
        data_handler: fn(buffer: Vec<u8>) -> Result<(), Box<dyn Error + Send>>,
        err_handler: fn(buffer: Vec<u8>) -> Result<(), Box<dyn Error + Send>>,
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

        // Start the data handler as Tokio task.
        let data_topic = client_config.data_channel();

        tokio::spawn(async move {
            if let Err(e) = handle_channel(&data_topic, data_handler).await {
                eprintln!("[QDClient/new]: Consumer connection error: {}", e);
            }
        });

        // Start the error handler as Tokio task.
        let err_topic = client_config.error_channel();
        tokio::spawn(async move {
            if let Err(e) = handle_channel(&err_topic, err_handler).await {
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

/// Handles incoming messages on a topic channel.
///
/// # Arguments
///
/// * `channel_topic` - The topic to subscribe to.
/// * `event_handler` - The callback to process each message.
///
/// For each message received on the topic, this:
///
/// - Creates a Fluvio consumer for the topic.
/// - Gets a stream for the consumer.
/// - Loops through records in the stream.
/// - Extracts the message bytes.
/// - Calls the `event_handler` with the message buffer.
///
async fn handle_channel(
    channel_topic: &str,
    event_handler: fn(buffer: Vec<u8>) -> Result<(), Box<dyn Error + Send>>,
) -> Result<(), Box<dyn Error + Send>> {
    // Create consumer for channel topic.
    let consumer = fluvio::consumer(channel_topic, 0)
        .await
        .expect("Failed to create a consumer for data topic");

    // Create stream for consumer.
    let mut stream = consumer
        .stream(Offset::end())
        .await
        .expect("Failed to create a stream");

    // Consume records from the stream and process with the event handler.
    while let Some(Ok(record)) = stream.next().await {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();

        event_handler(buffer.to_vec())?;
    }

    Ok(())
}
