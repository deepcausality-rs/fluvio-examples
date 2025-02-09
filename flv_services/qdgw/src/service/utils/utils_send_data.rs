use iggy::client::MessageClient;
use iggy::messages::send_messages::{Message, Partitioning, SendMessages};
use warp::hyper::body::Bytes;

use common::prelude::{MessageProcessingError, OHLCVBar, TradeBar};
use db_query_manager::types::{OHLCVRow, TradeRow};
use sbe_messages::prelude::{DataErrorType, DataType, SbeOHLCVBar, SbeTradeBar};

use crate::service::Server;

impl Server {
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

        // Build iggy message wrapper
        let message = Message::new(None, Bytes::from(enc_first_bar), None);

        // Send the first bar message to inform the client that the data stream starts
        match self.send_client_data(client_id, vec![message]).await {
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

        // Build iggy message wrapper
        let message = Message::new(None, Bytes::from(enc_last_bar), None);

        // Send the first bar message to inform the client that the data stream starts
        match self.send_client_data(client_id, vec![message]).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub(crate) async fn send_trade_bar(
        &self,
        client_id: u16,
        symbol_id: u16,
        record: &TradeRow,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        let bar = TradeBar::new(
            symbol_id,
            record.date_time(),
            record.price(),
            record.volume(),
        );
        // Encode the trade bar message
        let (_, enc_trade_bar) = SbeTradeBar::encode(bar).unwrap();

        // Build iggy message wrapper
        let message = Message::new(None, Bytes::from(enc_trade_bar), None);

        // Send trade bar message to inform the client that the data stream starts
        match self.send_client_data(client_id, vec![message]).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub(crate) async fn send_ohlcv_bar(
        &self,
        client_id: u16,
        symbol_id: u16,
        record: &OHLCVRow,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        let bar = OHLCVBar::new(
            symbol_id,
            record.date_time(),
            record.open(),
            record.high(),
            record.low(),
            record.close(),
            record.volume(),
        );
        // Encode the trade bar message
        let (_, enc_ohlcv_bar) = SbeOHLCVBar::encode(bar).unwrap();

        // Build iggy message wrapper
        let message = Message::new(None, Bytes::from(enc_ohlcv_bar), None);

        // Send the ohlcv bar message to inform the client that the data stream starts
        match self.send_client_data(client_id, vec![message]).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    /// Sends client data messages to the client's data channel.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The ID of the client to send the data to
    /// * `messages` - The vector of Message structs containing the data
    ///
    /// # Returns
    ///
    /// Returns a Result with `()` on success, or a (DataErrorType, MessageProcessingError) on failure.
    ///
    /// This function:
    ///
    /// - Locks the client_configs hashmap and gets the client's configuration
    /// - Locks the client_data_producers hashmap and gets the client's producer
    /// - Sends the messages to the client's topic/partition using the producer
    /// - Unlocks the hashmaps
    ///
    pub(crate) async fn send_client_data(
        &self,
        client_id: u16,
        messages: Vec<Message>,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Lock the client_configs hashmap
        let client_configs = self.client_configs().write().await;

        // Get the client config for the client
        let iggy_config = client_configs.get(&client_id).unwrap();

        // lock the client_data_producers hashmap
        let client_data_producers = self.client_producers().read().await;

        // Get the producer for the error channel
        let producer = client_data_producers
            .get(&client_id)
            .expect("[QDGW/utils_message::send_client_data]: No producer found");

        producer
            .send_messages(&mut SendMessages {
                stream_id: iggy_config.stream_id(),
                topic_id: iggy_config.topic_id(),
                partitioning: Partitioning::partition_id(iggy_config.partition_id()),
                messages,
            })
            .await
            .expect("Failed to send error");

        // Unlock the client_configs hashmap
        drop(client_configs);

        // Unlock the client_data_producers hashmap
        drop(client_data_producers);

        Ok(())
    }
}
