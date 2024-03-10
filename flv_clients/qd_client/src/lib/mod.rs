use std::error::Error;
use std::time::Duration;

use iggy::clients::client::IggyClient;
use iggy::messages::poll_messages::{PollingStrategy, PollMessages};
use tokio::time::sleep;

use common::prelude::{IggyConfig, IggyUser, ServiceID};

mod getters;
mod send_login;
mod send_logout;
mod send_start_data;
mod shared;

/// The QDClient struct.
pub struct QDClient {
    client_id: u16,
    producer: IggyClient,
    consumer: IggyClient,
    poll_command: PollMessages,
    iggy_config: IggyConfig,
}

impl QDClient {
    /// Creates a new QDClient instance.
    pub async fn new(
        client_id: u16,
        iggy_config: IggyConfig,
    ) -> Result<Self, Box<dyn Error + Send>> {
        // Producer is configured to send messages to the  QD gateway channel
        let producer_user = IggyUser::default();
        let producer_id = ServiceID::QDGW.id() as u32;
        let producer_config = IggyConfig::from_client_id(producer_user, producer_id, 50000, false);
        let producer = iggy_utils::get_producer(&producer_config)
            .await
            .expect("Failed to create iggy producer");

        // The poll command is using the producer config for polling for messages from the QD gateway
        let poll_command = get_poll_command(&producer_config);

        // Consumer is configured for listing for incoming messages on the client channel
        let consumer = iggy_utils::get_consumer(&iggy_config)
            .await
            .expect("Failed to create iggy consumer");

        // Create client.
        let client = Self {
            client_id,
            producer,
            consumer,
            poll_command,
            iggy_config,
        };

        // Login to the QD gateway and register the clients data channel
        client
            .login()
            .await
            .expect("[QDClient/new]: Failed to log in to the QD Gateway");

        Ok(client)
    }
}

// Preconfigure the poll message command for the consumer client
fn get_poll_command(iggy_config: &IggyConfig) -> PollMessages {
    PollMessages {
        consumer: Default::default(),
        stream_id: iggy_config.stream_id(),
        topic_id: iggy_config.topic_id(),
        partition_id: Option::from(iggy_config.partition_id()),
        strategy: PollingStrategy::last(),
        count: iggy_config.messages_per_batch(),
        auto_commit: iggy_config.auto_commit(),
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

        // Delete stream and topic before shutting down.
        iggy_utils::cleanup(&self.consumer(), &self.iggy_config())
            .await
            .expect("Failed to clean up iggy consumer");

        // Shutdown consumer
        iggy_utils::shutdown(&self.consumer())
            .await
            .expect("Failed to shutdown iggy consumer");

        // Shutdown producer
        iggy_utils::shutdown(&self.producer())
            .await
            .expect("Failed to shutdown iggy producer");

        Ok(())
    }
}
