use crate::prelude::{DataErrorType, MessageType};
use serde::{Deserialize, Serialize};

mod display;
mod getters;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct DataErrorMessage {
    message_type: MessageType,
    client_id: u16,
    data_error_type: DataErrorType,
}

impl DataErrorMessage {
    /// Creates a new DataErrorMessage instance.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Client ID
    /// * `data_error_type` - Data error type
    ///
    /// # Returns
    ///
    /// New DataErrorMessage instance
    ///
    /// # Remarks
    ///
    /// Sets message_type to DataError
    ///
    pub fn new(client_id: u16, data_error_type: DataErrorType) -> Self {
        let message_type = MessageType::DataError;
        Self {
            message_type,
            client_id,
            data_error_type,
        }
    }
}

impl From<&[u8]> for DataErrorMessage {
    /// Decodes a DataErrorMessage from a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte buffer containing encoded DataErrorMessage
    ///
    /// # Returns
    ///
    /// Decoded DataErrorMessage on success
    ///
    /// # Errors
    ///
    /// Returns Err if decoding fails
    ///
    /// # Remarks
    ///
    /// Calls sbe_decode::decode_data_error_message to decode message
    ///
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_client_error_message(value).expect("Failed to decode DataErrorMessage")
    }
}
