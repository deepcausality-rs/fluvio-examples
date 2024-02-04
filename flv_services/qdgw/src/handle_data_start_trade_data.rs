use crate::service::Server;
use common::prelude::{MessageProcessingError, TradeBar};
use sbe_messages::prelude::{DataErrorType, DataType, SbeTradeBar};
use futures::StreamExt;


impl Server {
    /// Sends a stream of trade bar data to the client.
    pub(crate) async fn start_trade_data(
        &self,
        client_id: u16,
        _first_bar: Vec<u8>,
        _data_bars: &[TradeBar],
        _last_bar: Vec<u8>,
        // symbol_id: u16,
        // trade_table: &str,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        let data_type = DataType::TradeData;

        //
        let symbol_id = 43;
        let trade_table = "trades";

        // Encode the first bar message
        let enc_first_bar = match self.encode_first_bar(&data_type, symbol_id).await {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        // Send the first bar message to inform the client that the data stream starts
        match self.send_single_data(client_id, enc_first_bar).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Lock the query manager
        let q_manager = self.query_manager.lock().await;

        // Create a stream of trade bars from the database
        let mut stream = q_manager.stream_trades(symbol_id, trade_table).await;

        // Process trade bars from the stream as they come in
        while let Some(Ok(record)) = stream.next().await {
            // Encode the trade bar message
            let (_, enc_trade_bar) = SbeTradeBar::encode(record).unwrap();

            // Send trade bar message to the client
            match self.send_single_data(client_id, enc_trade_bar).await {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        // Encode the last bar message
        let enc_last_bar = match self.encode_last_bar(&data_type, symbol_id).await {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        // Send the last bar message to inform the client that the data stream has ended
        match self.send_single_data(client_id, enc_last_bar).await {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }
}
