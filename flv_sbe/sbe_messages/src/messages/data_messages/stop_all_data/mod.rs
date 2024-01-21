use serde::{Deserialize, Serialize};

use common::prelude::ExchangeID;

use crate::prelude::MessageType;

mod display;
mod getter;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct StopAllDataMessage {
    message_type: MessageType,
    client_id: u16,
    exchange_id: ExchangeID,
}

impl StopAllDataMessage {
    /// Creates a new StopAllDataMessage instance.
    ///
    /// Sets the message_type to StopAllData.
    ///
    /// # Arguments
    ///
    /// * `client_id` - u16 client ID
    /// * `exchange_id` - ExchangeID exchange ID
    ///
    /// # Returns
    ///
    /// StopAllDataMessage instance
    ///
    pub fn new(client_id: u16, exchange_id: ExchangeID) -> Self {
        let message_type = MessageType::StopAllData;
        Self {
            message_type,
            client_id,
            exchange_id,
        }
    }
}

impl From<&[u8]> for StopAllDataMessage {
    /// Implements the From trait to decode a StopAllDataMessage from a byte slice.
    ///
    /// Calls the sbe_decode::decode_stop_all_data_message function to decode the message.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte slice to decode
    ///
    /// # Returns
    ///
    /// Decoded StopAllDataMessage
    ///
    /// # Errors
    ///
    /// Panics if decode fails
    ///
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_stop_all_data_message(value)
            .expect("Failed to decode start data message")
    }
}
