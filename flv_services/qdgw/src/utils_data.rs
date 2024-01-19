use crate::service::Server;
use common::prelude::{ClientChannel, MessageProcessingError, OHLCVBar, TimeResolution, TradeBar};
use db_query_manager::error::QueryError;
use fluvio::RecordKey;
use sbe_messages::prelude::DataErrorType;

impl Server {
    /// Gets all trade bars for the given symbol id and trade table.
    ///
    /// # Parameters
    ///
    /// * `symbol_id` - The numeric id of the symbol to get bars for.
    /// * `trade_table` - The name of the trade table to query.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with a `Vec` of `TradeBar` structs if successful,
    /// otherwise returns a `QueryError` on failure.
    ///
    pub(crate) async fn get_trade_bars(
        &self,
        symbol_id: u16,
        trade_table: &str,
    ) -> Result<Vec<TradeBar>, QueryError> {
        // Lock query manager
        let mut q_manager = self.query_manager.lock().await;

        // Get all bars
        let result = q_manager.get_all_trades(symbol_id, trade_table).await;

        // Unlock / drop query manager
        drop(q_manager);

        match result {
            Ok(bars) => Ok(bars),
            Err(e) => Err(e),
        }
    }

    /// Gets OHLCV bars for the given symbol id, time resolution and trade table.
    ///
    /// # Parameters
    ///
    /// * `symbol_id` - The numeric id of the symbol to get bars for.
    /// * `time_resolution` - The time resolution to use for the bars.
    /// * `trade_table` - The name of the trade table to query.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with a `Vec` of `OHLCVBar` structs if successful,
    /// otherwise returns a `QueryError` on failure.
    ///
    pub(crate) async fn get_ohlcv_bars(
        &self,
        symbol_id: u16,
        time_resolution: &TimeResolution,
        trade_table: &str,
    ) -> Result<Vec<OHLCVBar>, QueryError> {
        // Lock query manager
        let mut q_manager = self.query_manager.lock().await;

        // Get all bars
        let result = q_manager
            .get_all_ohlcv_bars(symbol_id, trade_table, time_resolution)
            .await;

        // Unlock / drop query manager
        drop(q_manager);

        match result {
            Ok(bars) => Ok(bars),
            Err(e) => Err(e),
        }
    }

    /// Sends the provided data buffer to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the data to.
    /// * `buffer` - The data buffer to send.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `(DataErrorType, MessageProcessingError)` tuple containing:
    ///
    /// - `DataErrorType::DataSendError`
    /// - The underlying send error wrapped in `MessageProcessingError`
    ///
    pub(crate) async fn send_data(
        &self,
        client_id: u16,
        buffer: Vec<u8>,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Get the producer for the error channel
        let producer = self
            .get_channel_producer(ClientChannel::ErrorChannel, client_id)
            .await
            .expect("[send_error]: Failed to get error channel producer");

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
}
