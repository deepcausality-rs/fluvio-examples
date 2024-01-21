use crate::prelude::{ClientErrorType, MessageType};
use serde::{Deserialize, Serialize};

mod display;
mod getters;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ClientErrorMessage {
    message_type: MessageType,
    client_id: u16,
    client_error_type: ClientErrorType,
}

impl ClientErrorMessage {
    /// Creates a new ClientErrorMessage instance.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Client ID
    /// * `client_error_type` - Client error type
    ///
    /// # Returns
    ///
    /// New ClientErrorMessage instance
    ///
    /// # Remarks
    ///
    /// Sets message_type to ClientError
    ///
    pub fn new(client_id: u16, client_error_type: ClientErrorType) -> Self {
        let message_type = MessageType::ClientError;
        Self {
            message_type,
            client_id,
            client_error_type,
        }
    }
}

impl From<&[u8]> for ClientErrorMessage {
    /// Decodes a ClientErrorMessage from a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte buffer containing encoded ClientErrorMessage
    ///
    /// # Returns
    ///
    /// Decoded ClientErrorMessage on success
    ///
    /// # Errors
    ///
    /// Returns Err if decoding fails
    ///
    /// # Remarks
    ///
    /// Calls sbe_decode::decode_client_error_message to decode message
    ///
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_client_error_message(value).expect("Failed to decode ClientLoginMessage")
    }
}
