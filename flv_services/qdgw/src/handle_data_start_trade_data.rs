use crate::service::Server;
use common::prelude::{MessageProcessingError, TradeBar};
use futures::StreamExt;
use sbe_messages::prelude::{DataErrorType, DataType, SbeTradeBar};

impl Server {
    /// Sends a stream of trade bar data to the client.
    ///
    /// This will:
    ///
    /// 1. Send a first trade bar message to indicate the start of the stream.
    /// 2. Stream trade bars from the database for the given symbol.
    /// 3. Encode each trade bar into an SBE message.
    /// 4. Send the encoded trade bar messages to the client.
    /// 5. Send a last trade bar message to indicate the end of the stream.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The id of the client to stream trade bars to.
    /// * `symbol_id` - The symbol id to stream trade bars for.
    /// * `trade_table` - The database table to stream trade bars from.
    ///
    /// # Errors
    ///
    /// Returns a Result with the error variants:
    ///
    /// - `(DataErrorType, MessageProcessingError)` - Error streaming trade bars.
    ///
    /// # Example
    ///
    /// ```
    /// use flv_services::qdgw::Server;
    /// async fn example(server: &Server)
    ///         -> Result<(), (DataErrorType, MessageProcessingError)> {
    ///
    ///     let client_id = 1;
    ///     let symbol_id = 2;
    ///     let trade_table = "trades";
    ///
    ///     server.start_trade_data(client_id, symbol_id, trade_table).await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    pub(crate) async fn start_trade_data_stream(
        &self,
        client_id: u16,
        symbol_id: u16,
        trade_table: &str,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Set the data type to trade data
        let data_type = DataType::TradeData;
        // Disables the flushing after each message thus increasing throughput by batch sending.
        let flush = false;

        // Send the first bar message to inform the client that the data stream starts
        match self.send_first_bar(client_id, symbol_id, &data_type).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Lock the query manager
        let q_manager = self.query_manager().read().await;

        // Create a stream of trade bars from the database
        let mut stream = q_manager.stream_trades(&trade_table).await;

        // Process trade bars from the stream as they come in
        while let Some(Ok(record)) = stream.next().await {
            let bar = TradeBar::new(
                symbol_id,
                record.date_time(),
                record.price(),
                record.volume(),
            );
            // Encode the trade bar message
            let (_, enc_trade_bar) = SbeTradeBar::encode(bar).unwrap();

            // Send trade bar message to the client
            match self.send_single_data(client_id, enc_trade_bar, flush).await {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        // Send the last bar message to inform the client that the data stream has ended
        match self.send_last_bar(client_id, symbol_id, &data_type).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }
}
