use crate::service::Server;
use common::prelude::{ClientChannel, MessageProcessingError};
use fluvio::{Fluvio, RecordKey};
use sbe_messages::prelude::{DataErrorMessage, DataErrorType, FirstTradeBar, LastTradeBar, SbeTradeBar, StartDataMessage};

// Rewrite this:
// Handle all errors in the handler & return error messages to client
// Construct a dedicated producer for the client control channel
// Only send data in the start_data
// Create a second producer for the data channel
//
impl Server {
    pub(crate) async fn handle_start_data_message(
        &self,
        start_data_msg: &StartDataMessage,
    ) -> Result<(), MessageProcessingError> {
        //
        let client_id = start_data_msg.client_id();
        let exchange_id = *start_data_msg.exchange_id();
        let symbol_id = start_data_msg.symbol_id();

        let trade_table = match self.get_trade_table_name(exchange_id).await {
            Ok(table) => table,
            Err(e) => {
                // Send error message back to client instead of return
                return Err(e);
            }
        };

        let client_data_channel = match self
            .get_client_channel(ClientChannel::DataChannel, *client_id)
            .await
        {
            Ok(channel) => channel,
            Err(e) => {
                return Err(e);
            }
        };

        self.start_data(client_id, &client_data_channel, *symbol_id, &trade_table)
            .await
    }
// pass in a reference to the data bars
    pub(crate) async fn start_data(
        &self,
        client_id: &u16,
        client_data_channel: &str,
        symbol_id: u16,
        trade_table: &str,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!(
            "[QDGW/handle::start_date]:on channel : {:?}",
            client_data_channel
        );


        let _client_control_channel = match self
            .get_client_channel(ClientChannel::ControlChannel, *client_id)
            .await
        {
            Ok(channel) => channel,
            Err(e) => {
                return Err(e);
            }
        };

        let fluvio = Fluvio::connect().await.unwrap();

        let producer = fluvio
            .topic_producer(client_data_channel)
            .await
            .expect("Failed to create a producer");

        // Lock query manager
        let mut q_manager = self.query_manager.lock().await;

        // Get all bars
        let result = q_manager.get_all_trades(trade_table).await;

        // Handle error
        let bars = match result {
            Ok(bars) => bars,
            Err(err) => {
                println!("[QDGW/handle::start_date]: Error getting bars: {:?}", err);

                let data_error_type = DataErrorType::DataUnavailableError;
                let message = DataErrorMessage::new(symbol_id, data_error_type);

                let enc = message.encode();
                assert!(enc.is_ok());
                let (_, buffer) = enc.unwrap();

                producer
                    .send(RecordKey::NULL, buffer)
                    .await
                    .expect("Failed to send DataError: DataUnavailableError!");
                producer.flush().await.expect("Failed to flush");

            }
        };

        // Send first  bar message to inform the client that the data stream starts
        let first_bar = FirstTradeBar::new(symbol_id);
        let (_, buffer) = first_bar
            .encode()
            .expect("Failed to encode first trade bar");

        producer
            .send(RecordKey::NULL, buffer)
            .await
            .expect("Failed to send Done!");
        producer.flush().await.expect("Failed to flush");

        for bar in bars {
            let (_, buffer) =
                SbeTradeBar::encode_data_bar_message(bar).expect("Failed to encode trade bar");

            producer
                .send(RecordKey::NULL, buffer)
                .await
                .expect("Failed to send Done!");
        }

        // Send last bar message to inform the client that the data stream has ended
        let last_bar = LastTradeBar::new(symbol_id);
        let (_, buffer) = last_bar.encode().expect("Failed to encode last trade bar");

        producer
            .send(RecordKey::NULL, buffer)
            .await
            .expect("Failed to send Done!");
        producer.flush().await.expect("Failed to flush");

        Ok(())
    }
}
