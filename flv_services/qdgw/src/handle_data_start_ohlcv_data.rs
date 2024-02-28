use crate::service::Server;
use common::prelude::{MessageProcessingError, OHLCVBar, TimeResolution};
use futures::StreamExt;
use sbe_messages::prelude::{DataErrorType, DataType, SbeOHLCVBar};

impl Server {
    /// Sends a stream of OHLCV bar data to the client.
    ///
    /// This will:
    ///
    /// 1. Send a first OHLCV bar message to indicate the start of the stream.
    /// 2. Stream OHLCV bars from the database for the given symbol and time resolution.
    /// 3. Encode each OHLCV bar into an SBE message.
    /// 4. Send the encoded OHLCV bar messages to the client.
    /// 5. Send a last OHLCV bar message to indicate the end of the stream.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The id of the client to stream OHLCV bars to.
    /// * `symbol_id` - The symbol id to stream OHLCV bars for.
    /// * `trade_table` - The database table to stream OHLCV bars from.
    /// * `time_resolution` - The time resolution of the OHLCV bars.
    ///
    /// # Errors
    ///
    /// Returns a Result with the error variants:
    ///
    /// - `(DataErrorType, MessageProcessingError)` - Error streaming OHLCV bars.
    ///
    /// # Example
    ///
    /// ```
    /// use flv_services::qdgw::Server;
    /// use common::prelude::TimeResolution;
    /// async fn example(server: &Server)
    ///     -> Result<(), (DataErrorType, MessageProcessingError)> {
    ///
    ///     let client_id = 1;
    ///     let symbol_id = 2;
    ///     let trade_table = "trades";
    ///     let time_resolution = TimeResolution::OneMinute;
    ///
    ///     server
    ///     .start_ohlcv_data(client_id, symbol_id, trade_table, &time_resolution)
    ///     .await?;
    ///
    /// Ok(())
    /// }
    /// ```
    ///
    pub(crate) async fn start_ohlcv_data(
        &self,
        client_id: u16,
        symbol_id: u16,
        trade_table: &str,
        time_resolution: &TimeResolution,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Set the data type to OHLCV
        let data_type = DataType::OHLCVData;
        // Disables the flushing after each message thus increasing throughput by batch sending.
        let flush = false;

        // Send the first bar message to inform the client that the data stream starts
        match self.send_first_bar(client_id, symbol_id, &data_type).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // Lock the query manager
        let q_manager = self.query_manager.read().await;

        // Create a stream of trade bars from the database
        let mut stream = q_manager.stream_ohlcv(trade_table, time_resolution).await;

        // Process OHLCV bars from the stream as they come in
        while let Some(Ok(record)) = stream.next().await {
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
            let (_, enc_trade_bar) = SbeOHLCVBar::encode(bar).unwrap();

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
