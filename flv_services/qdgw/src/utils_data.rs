use crate::service::Server;
use common::prelude::{MessageProcessingError, OHLCVBar, TimeResolution, TradeBar};
use db_query_manager::error::QueryError;
use fluvio::{RecordKey, TopicProducer};
use sbe_messages::prelude::{ClientErrorMessage, ClientErrorType, DataErrorMessage, DataErrorType};

impl Server {
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
        producer: &TopicProducer,
        buffer: Vec<u8>,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        match producer.send(RecordKey::NULL, buffer).await {
            Ok(_) => {}
            Err(e) => {
                return Err((
                    DataErrorType::DataSendError,
                    MessageProcessingError(e.to_string()),
                ));
            }
        }
        producer.flush().await.expect("Failed to flush");

        Ok(())
    }

    /// Sends a ClientError message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `client_id` - The id of the client the error is for
    /// * `client_error` - The ClientErrorType to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_client_error(
        &self,
        producer: &TopicProducer,
        client_id: u16,
        client_error: ClientErrorType,
    ) -> Result<(), MessageProcessingError> {
        let message = ClientErrorMessage::new(client_id, client_error);
        let enc = message.encode();
        assert!(enc.is_ok());
        let (_, buffer) = enc.unwrap();

        self.send_error(producer, buffer).await?;

        Ok(())
    }

    /// Sends a DataError message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `client_id` - The id of the client the error is for
    /// * `data_error` - The DataErrorType to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_data_error(
        &self,
        producer: &TopicProducer,
        client_id: u16,
        data_error: DataErrorType,
    ) -> Result<(), MessageProcessingError> {
        let message = DataErrorMessage::new(client_id, data_error);
        let enc = message.encode();
        assert!(enc.is_ok());
        let (_, buffer) = enc.unwrap();

        self.send_error(producer, buffer).await?;

        Ok(())
    }

    /// Sends an error message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `error_buffer` - The encoded error message bytes to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_error(
        &self,
        producer: &TopicProducer,
        error_buffer: Vec<u8>,
    ) -> Result<(), MessageProcessingError> {
        producer
            .send(RecordKey::NULL, error_buffer)
            .await
            .expect("Failed to send DataError: DataUnavailableError!");
        producer
            .flush()
            .await
            .expect("Failed to flush to message bus.");

        Ok(())
    }

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
}
