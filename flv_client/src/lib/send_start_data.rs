use crate::QDClient;
use common::prelude::{ExchangeID, TimeResolution};
use sbe_messages::prelude::{DataType, StartDataMessage};
use std::error::Error;

impl QDClient {
    /// Sends a StartDataMessage to request trade data.
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The exchange ID for the symbol.
    /// * `symbol_id` - The symbol ID to request data for.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` on success, or an `Error` on failure.
    ///
    /// This creates a `StartDataMessage` requesting `TradeData` for the
    /// given symbol. It encodes the message and sends it to the gateway.
    ///
    pub async fn start_trade_data(
        &self,
        exchange_id: ExchangeID,
        symbol_id: u16,
    ) -> Result<(), Box<dyn Error + Send>> {
        // Create message
        let data_type = DataType::TradeData;
        let time_resolution = TimeResolution::NoValue; // Time resolution will be ignored for TradeData hence NoValue.
        let message = StartDataMessage::new(
            self.client_id,
            exchange_id,
            symbol_id,
            time_resolution,
            data_type,
        );

        // Encode message
        let (_, buffer) = message
            .encode()
            .expect("[QDClient/start_trade_data]: Failed to encode message");

        // Send message to the gateway
        self.send_message(buffer)
            .await
            .expect("[QDClient/start_trade_data]: Failed to send StartDataMessage message!");

        Ok(())
    }

    /// Sends a StartDataMessage to request OHLCV data.
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The exchange ID for the symbol.
    /// * `symbol_id` - The symbol ID to request data for.
    /// * `time_resolution` - The time resolution for the OHLCV data.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` on success, or an `Error` on failure.
    ///
    /// This creates a `StartDataMessage` requesting `OHLCVData` for the given
    /// symbol and time resolution. It encodes the message and sends it to
    /// the gateway.
    ///
    pub async fn start_ohlcv_data(
        &self,
        exchange_id: ExchangeID,
        symbol_id: u16,
        time_resolution: TimeResolution,
    ) -> Result<(), Box<dyn Error + Send>> {
        // Create message
        let data_type = DataType::OHLCVData;
        let message = StartDataMessage::new(
            self.client_id,
            exchange_id,
            symbol_id,
            time_resolution,
            data_type,
        );

        // Encode message
        let (_, buffer) = message
            .encode()
            .expect("[QDClient/start_ohlcv_data]: Failed to encode message");

        // Send message to the gateway
        self.send_message(buffer)
            .await
            .expect("[QDClient/start_ohlcv_data]: Failed to send StartDataMessage message!");

        Ok(())
    }
}
