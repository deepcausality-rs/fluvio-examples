use common::prelude::{IggyConfig, IggyUser, MessageClientConfig};
use iggy::clients::client::IggyClient;
use iggy::messages::poll_messages::{PollingStrategy, PollMessages};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

mod send_login;
mod send_logout;
mod send_start_data;
mod shared;
mod getters;

/// The QDClient struct.
pub struct QDClient {
    client_id: u16,
    producer: IggyClient,
    consumer: IggyClient,
    poll_command: PollMessages,
    client_config: MessageClientConfig,
    iggy_config: IggyConfig,
}


impl QDClient {
    /// Creates a new QDClient instance.
    pub async fn new(
        client_id: u16,
        client_config: MessageClientConfig,
        iggy_config: IggyConfig,
    ) -> Result<Self, Box<dyn Error + Send>> {

        // Preconfigure the poll message command for the consumer client
        let poll_command = PollMessages {
            consumer: Default::default(),
            stream_id: iggy_config.stream_id(),
            topic_id: iggy_config.topic_id(),
            partition_id: Option::from(iggy_config.partition_id()),
            strategy: PollingStrategy::last(),
            count: iggy_config.messages_per_batch(),
            auto_commit: iggy_config.auto_commit(),
        };

        // Move authentication info into the iggy config
        let user = IggyUser::default();

        // Create an iggy client and initialize it as consumer
        let consumer = iggy_utils::get_consumer(&iggy_config, &user)
            .await
            .expect("Failed to create consumer client");

        // Create an iggy client and initialize it as producer
        let producer = iggy_utils::get_producer(&iggy_config, &user)
            .await
            .expect("Failed to create producer client");

        // Create client.
        let client = Self {
            client_id,
            producer,
            consumer,
            poll_command,
            client_config,
            iggy_config,
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

        // Delete stream and topic before shutting down.
        iggy_utils::cleanup(&self.consumer(), &self.iggy_config())
            .await
            .expect("Failed to clean up iggy");

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
