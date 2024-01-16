use crate::service::Server;
use common::prelude::{ClientChannel, MessageProcessingError, TradeBar};
use fluvio::{Fluvio, RecordKey};
use sbe_messages::prelude::{DataErrorMessage, DataErrorType, DataType, FirstTradeBar, LastTradeBar, SbeTradeBar, StartDataMessage};

impl Server {
    pub(crate) async fn handle_start_data_message(
        &self,
        start_data_msg: &StartDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!(
            "[QDGW/handle::handle_start_data_message]: {:?}",
            &start_data_msg
        );

        // Extract fields from message
        let client_id = *start_data_msg.client_id();
        let exchange_id = *start_data_msg.exchange_id() as u8;
        let symbol_id = *start_data_msg.symbol_id();
        let data_type = start_data_msg.data_type_id();

        // Get the client's control channel to return error messages back to the client
        let client_control_channel = match self
            .get_client_channel(ClientChannel::ControlChannel, client_id)
            .await
        {
            Ok(channel) => channel,
            Err(e) => {
                return Err(e);
            }
        };

        // Connect to the Fluvio cluster
        let fluvio = Fluvio::connect().await.unwrap();

        // Get the producer for the client's control channel
        let producer = fluvio
            .topic_producer(client_control_channel)
            .await
            .expect("Failed to create a producer");

        let trade_table = match self.get_trade_table_name(exchange_id).await {
            Ok(table) => table,
            Err(_) => {
                let data_error = DataErrorType::DataTableNotFound;
                let message = DataErrorMessage::new(client_id, data_error);

                let enc = message.encode();
                assert!(enc.is_ok());
                let (_, buffer) = enc.unwrap();

                producer
                    .send(RecordKey::NULL, buffer)
                    .await
                    .expect("Failed to send DataError: DataTableNotFound!");
                producer.flush().await.expect("Failed to flush");

                return Err(MessageProcessingError("Failed to get dat table".into()));
            }
        };

        match data_type {
            DataType::UnknownDataType => {
                let data_error = DataErrorType::DataTypeNotKnownError;
                let message = DataErrorMessage::new(client_id, data_error);

                let enc = message.encode();
                assert!(enc.is_ok());
                let (_, buffer) = enc.unwrap();

                producer
                    .send(RecordKey::NULL, buffer)
                    .await
                    .expect("Failed to send DataError: DataTypeNotKnownError!");
                producer.flush().await.expect("Failed to flush");
            }
            DataType::TradeData => {
                // Get all bars
                let result = self.get_trade_bars(symbol_id, &trade_table).await;

                // Handle query error error
                let bars = match result {
                    Ok(bars) => bars,
                    Err(err) => {
                        let data_error = DataErrorType::DataUnavailableError;
                        let message = DataErrorMessage::new(client_id, data_error);
                        let enc = message.encode();
                        assert!(enc.is_ok());
                        let (_, buffer) = enc.unwrap();

                        producer
                            .send(RecordKey::NULL, buffer)
                            .await
                            .expect("Failed to send DataError: DataUnavailableError!");
                        producer.flush().await.expect("Failed to flush");

                        return Err(MessageProcessingError("Failed to get bars".into()));
                    }
                };

                match self.start_trade_data(client_id, symbol_id, &bars).await {
                    Ok(_) => {}
                    Err(err) => {
                        // Add proper error handling here & send error message to client
                    }
                }
            }
            DataType::OHLCVData => {}
        };


        Ok(())
    }

    // pass in a reference to the data bars
    pub(crate) async fn start_trade_data(
        &self,
        client_id: u16,
        symbol_id: u16,
        trade_bars: &Vec<TradeBar>,
    ) -> Result<(), MessageProcessingError> {

        // Get the clients data channel to send data back to the client.
        let client_data_channel = match self
            .get_client_channel(ClientChannel::DataChannel, client_id)
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

        for bar in trade_bars.to_vec() {
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
