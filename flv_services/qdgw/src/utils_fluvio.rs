use crate::service::Server;
use common::prelude::{
    ClientChannel, MessageClientConfig, MessageProcessingError, OHLCVBar, TradeBar,
};
use fluvio::{Fluvio, RecordKey, TopicProducer};
use sbe_messages::prelude::{DataErrorType, SbeOHLCVBar, SbeTradeBar};

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
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // lock the client_data_producers hashmap
        let client_data_producers = self.client_data_producers.lock().await;

        // Get the producer for the error channel
        let producer = client_data_producers
            .get(&client_id)
            .expect("No producer found");

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

        // Flush the producer
        producer.flush().await.expect("Failed to flush");

        Ok(())
    }

    /// Send a bulk set of trade bars to the data channel of the given client ID.
    ///
    /// Looks up the data producer for the client ID. Encodes and sends each
    /// trade bar to the producer. Flushes the producer after sending all bars.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to send the data to
    /// - `data_bars`: Vector of TradeBar structs to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` on success, or a `(DataErrorType, MessageProcessingError)`
    /// on failure to encode, send, or flush the data.
    ///
    pub(crate) async fn send_bulk_trade_data(
        &self,
        client_id: u16,
        data_bars: &Vec<TradeBar>,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // lock the client_data_producers hashmap
        let client_data_producers = self.client_data_producers.lock().await;

        // Get the producer for the error channel
        let producer = client_data_producers
            .get(&client_id)
            .expect("No producer found");

        for bar in data_bars.to_vec() {
            // Encode bar message
            let (_, buffer) = match SbeTradeBar::encode_data_bar_message(bar) {
                Ok(enc) => enc,
                Err(e) => {
                    return Err((
                        DataErrorType::DataEncodingError,
                        MessageProcessingError(e.to_string()),
                    ));
                }
            };

            // Send bar message to client
            match producer.send(RecordKey::NULL, buffer).await {
                Ok(_) => {}
                Err(e) => {
                    return Err((
                        DataErrorType::DataSendError,
                        MessageProcessingError(e.to_string()),
                    ));
                }
            }
        }

        // Flush out the producer
        producer.flush().await.expect("Failed to flush");

        Ok(())
    }

    /// Send a bulk set of OHLCV bars to the data channel of the given client ID.
    ///
    /// Looks up the data producer for the client ID. Encodes and sends each
    /// OHLCV bar to the producer. Flushes the producer after sending all bars.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to send the data to
    /// - `ohlcv_bars`: Vector of OHLCVBar structs to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` on success, or a `(DataErrorType, MessageProcessingError)`
    /// on failure to encode, send, or flush the data.
    ///
    pub(crate) async fn send_bulk_ohlcv_data(
        &self,
        client_id: u16,
        ohlcv_bars: &Vec<OHLCVBar>,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // lock the client_data_producers hashmap
        let client_data_producers = self.client_data_producers.lock().await;

        // Get the producer for the error channel
        let producer = client_data_producers
            .get(&client_id)
            .expect("No producer found");

        // Send all trade bars to the client
        for bar in ohlcv_bars.to_vec() {
            // Encode bar message
            let (_, buffer) = match SbeOHLCVBar::encode_data_bar_message(bar) {
                Ok(enc) => enc,
                Err(e) => {
                    return Err((
                        DataErrorType::DataEncodingError,
                        MessageProcessingError(e.to_string()),
                    ));
                }
            };

            // Send bar message to client
            match producer.send(RecordKey::NULL, buffer).await {
                Ok(_) => {}
                Err(e) => {
                    return Err((
                        DataErrorType::DataSendError,
                        MessageProcessingError(e.to_string()),
                    ));
                }
            }
        }

        // Flush out the producer
        producer.flush().await.expect("Failed to flush");

        Ok(())
    }
}
