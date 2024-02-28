use crate::service::Server;
use common::prelude::{ClientChannel, MessageClientConfig, MessageProcessingError};
use fluvio::{Fluvio, RecordKey, TopicProducer};
use sbe_messages::prelude::{DataErrorType, DataType};

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
            .expect("[QDGW/utils_fluvio::get_channel_producer]: Failed to create a producer");

        Ok(producer)
    }

    /// Sends a first bar message to the client to indicate the start of a data stream.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The id of the client to send the message to.
    /// * `symbol_id` - The symbol id for the first bar message.
    /// * `data_type` - The data type (OHLCV or Trade) for encoding the first bar message.
    ///
    /// # Errors
    ///
    /// Returns a Result with the error variants:
    ///
    /// - `(DataErrorType, MessageProcessingError)` - Error encoding or sending the first bar message.
    ///
    pub(crate) async fn send_first_bar(
        &self,
        client_id: u16,
        symbol_id: u16,
        data_type: &DataType,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Encode the first bar message
        let enc_first_bar = match self.encode_first_bar(data_type, symbol_id).await {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        // Send the first bar message to inform the client that the data stream starts
        match self.send_single_data(client_id, enc_first_bar, true).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    /// Sends a last bar message to the client to indicate the end of a data stream.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The id of the client to send the message to.
    /// * `symbol_id` - The symbol id for the last bar message.
    /// * `data_type` - The data type (OHLCV or Trade) for encoding the last bar message.
    ///
    /// # Errors
    ///
    /// Returns a Result with the error variants:
    ///
    /// - `(DataErrorType, MessageProcessingError)` - Error encoding or sending the last bar message.
    ///
    pub(crate) async fn send_last_bar(
        &self,
        client_id: u16,
        symbol_id: u16,
        data_type: &DataType,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Encode the last bar message
        let enc_last_bar = match self.encode_last_bar(data_type, symbol_id).await {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        // Send the last bar message to inform the client that the data stream has ended
        match self.send_single_data(client_id, enc_last_bar, true).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    /// Send a single data buffer to the data channel of the given client ID.
    ///
    /// Looks up the data producer for the client ID, sends the provided
    /// buffer, and flushes the producer.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to send the data to
    /// - `buffer`: The data buffer to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` on success, or a `(DataErrorType, MessageProcessingError)`
    /// on failure to send the data or flush the producer.
    ///
    pub(crate) async fn send_single_data(
        &self,
        client_id: u16,
        buffer: Vec<u8>,
        flush: bool,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // lock the client_data_producers hashmap
        let client_data_producers = self.client_data_producers.read().await;

        // Get the producer for the error channel
        let producer = client_data_producers
            .get(&client_id)
            .expect("[QDGW/utils_fluvio::send_single_data]: No producer found");

        // Send the data
        match producer.send(RecordKey::NULL, buffer).await {
            Ok(_) => {}
            Err(e) => {
                return Err((
                    DataErrorType::DataSendError,
                    MessageProcessingError(e.to_string()),
                ));
            }
        }

        if flush {
            // Flush the producer
            producer
                .flush()
                .await
                .expect("[QDGW/utils_fluvio::send_single_data]: Failed to flush");
        }

        Ok(())
    }
}
