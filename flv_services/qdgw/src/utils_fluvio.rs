use crate::service::Server;
use common::prelude::{ClientChannel, MessageClientConfig, MessageProcessingError};
use fluvio::{Fluvio, TopicProducer};

impl Server {
    /// Get the Fluvio channel name for the given client channel type and ID.
    ///
    /// Uses the MessageClientConfig to look up the channel name based on the
    /// provided ClientChannel enum variant and client ID.
    ///
    /// # Parameters
    ///
    /// - `client_channel` - The ClientChannel enum variant
    /// - `client_id` - The client ID
    ///
    /// # Returns
    ///
    /// The channel name string for the given channel and ID.
    /// Returns a MessageProcessingError if lookup fails.
    ///
    pub(crate) async fn get_client_channel(
        &self,
        client_channel: ClientChannel,
        client_id: u16,
    ) -> Result<String, MessageProcessingError> {
        let client_config = MessageClientConfig::new(client_id);

        // Look up the channel
        let channel = match client_channel {
            ClientChannel::DataChannel => client_config.data_channel(),
            ClientChannel::ErrorChannel => client_config.error_channel(),
            ClientChannel::ControlChannel => client_config.control_channel(),
            ClientChannel::ExecutionChannel => client_config.execution_channel(),
            ClientChannel::HeartbeatChannel => client_config.heartbeat_channel(),
        };

        Ok(channel)
    }

    /// Get a Fluvio producer for the error channel of the given client ID.
    ///
    /// Looks up the error channel name for the client ID using
    /// get_client_channel(). Connects to Fluvio and creates a producer
    /// for the error channel.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to get the error producer for
    ///
    /// # Returns
    ///
    /// - `Result<TopicProducer, MessageProcessingError>`: The Fluvio producer
    ///   for the client error channel if successful, otherwise a
    ///   MessageProcessingError on failure to get the channel or create the producer.
    ///
    pub(crate) async fn get_channel_producer(
        &self,
        client_channel: ClientChannel,
        client_id: u16,
    ) -> Result<TopicProducer, MessageProcessingError> {
        let client_error_channel = match self.get_client_channel(client_channel, client_id).await {
            Ok(channel) => {
                // println!("[get_error_channel_producer]: Got the client's error channel");
                channel
            }
            Err(e) => {
                // println!("[get_error_channel_producer]: Failed to get the client's error channel");
                return Err(e);
            }
        };

        let fluvio = Fluvio::connect().await.unwrap();

        let producer = fluvio
            .topic_producer(client_error_channel)
            .await
            .expect("[send_client_error]: Failed to create a producer");

        Ok(producer)
    }
}
