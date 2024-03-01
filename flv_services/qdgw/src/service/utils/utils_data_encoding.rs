use crate::service::Server;
use common::prelude::MessageProcessingError;
use sbe_messages::prelude::{
    DataErrorType, DataType, FirstOHLCVBar, FirstTradeBar, LastOHLCVBar, LastTradeBar,
};

impl Server {
    /// Encodes a FirstTradeBar or FirstOHLCVBar message for the given symbol id and data type.
    ///
    /// # Parameters
    ///
    /// * `date_type` - The data type to encode, TradeData or OHLCVData
    /// * `symbol_id` - The numeric id of the symbol
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the encoded message buffer if successful, otherwise returns a
    /// `(DataErrorType, MessageProcessingError)` tuple containing:
    ///
    /// - `DataErrorType::DataTypeNotKnownError` if the data type is unknown
    /// - `DataErrorType::DataEncodingError` if encoding fails
    /// - The underlying encoding error wrapped in `MessageProcessingError`
    ///
    pub(crate) async fn encode_first_bar(
        &self,
        date_type: &DataType,
        symbol_id: u16,
    ) -> Result<Vec<u8>, (DataErrorType, MessageProcessingError)> {
        match date_type {
            DataType::UnknownDataType => Err((
                DataErrorType::DataTypeNotKnownError,
                MessageProcessingError("DataTypeNotKnownError".to_string()),
            )),
            DataType::TradeData => {
                let first_trade_bar = FirstTradeBar::new(symbol_id);
                match first_trade_bar.encode() {
                    Ok((_, buf)) => Ok(buf),
                    Err(e) => Err((
                        DataErrorType::DataEncodingError,
                        MessageProcessingError(e.to_string()),
                    )),
                }
            }
            DataType::OHLCVData => {
                let first_ohlcv_bar = FirstOHLCVBar::new(symbol_id);
                match first_ohlcv_bar.encode() {
                    Ok((_, buf)) => Ok(buf),
                    Err(e) => Err((
                        DataErrorType::DataEncodingError,
                        MessageProcessingError(e.to_string()),
                    )),
                }
            }
        }
    }

    /// Encodes a LastTradeBar or LastOHLCVBar message for the given symbol id and data type.
    ///
    /// # Parameters
    ///
    /// * `date_type` - The data type to encode, TradeData or OHLCVData
    /// * `symbol_id` - The numeric id of the symbol
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the encoded message buffer if successful, otherwise returns a
    /// `(DataErrorType, MessageProcessingError)` tuple containing:
    ///
    /// - `DataErrorType::DataTypeNotKnownError` if the data type is unknown
    /// - `DataErrorType::DataEncodingError` if encoding fails
    /// - The underlying encoding error wrapped in `MessageProcessingError`
    ///
    pub(crate) async fn encode_last_bar(
        &self,
        date_type: &DataType,
        symbol_id: u16,
    ) -> Result<Vec<u8>, (DataErrorType, MessageProcessingError)> {
        match date_type {
            DataType::UnknownDataType => Err((
                DataErrorType::DataTypeNotKnownError,
                MessageProcessingError("DataTypeNotKnownError".to_string()),
            )),
            DataType::TradeData => {
                // Encode last trade bar message
                let last_trade_bar = LastTradeBar::new(symbol_id);
                match last_trade_bar.encode() {
                    Ok((_, buf)) => Ok(buf),
                    Err(e) => Err((
                        DataErrorType::DataEncodingError,
                        MessageProcessingError(e.to_string()),
                    )),
                }
            }
            DataType::OHLCVData => {
                // Encode last bar message
                let last_ohlcv_bar = LastOHLCVBar::new(symbol_id);
                match last_ohlcv_bar.encode() {
                    Ok((_, buf)) => Ok(buf),
                    Err(e) => Err((
                        DataErrorType::DataEncodingError,
                        MessageProcessingError(e.to_string()),
                    )),
                }
            }
        }
    }
}
