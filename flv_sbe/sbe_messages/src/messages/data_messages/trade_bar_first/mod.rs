use crate::prelude::MessageType;
use serde::{Deserialize, Serialize};

mod display;
mod getters;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FirstTradeBar {
    message_type: MessageType,
    symbol_id: u16,
}

impl FirstTradeBar {
    /// Creates a new FirstTradeBar instance.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - Symbol ID for the bar
    ///
    /// # Returns
    ///
    /// New FirstTradeBar instance
    ///
    /// # Remarks
    ///
    /// Sets message_type to FirstTradeBar

    pub fn new(symbol_id: u16) -> Self {
        let message_type = MessageType::FirstTradeBar;
        Self {
            message_type,
            symbol_id,
        }
    }
}

impl From<&[u8]> for FirstTradeBar {
    /// Decodes a FirstTradeBar message from a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte buffer containing encoded FirstTradeBar message
    ///
    /// # Returns
    ///
    /// Decoded FirstTradeBar on success
    ///
    /// # Errors
    ///
    /// Returns Err if decoding fails
    ///
    /// # Remarks
    ///
    /// Calls sbe_decode::decode_first_data_bar_message to decode message
    ///
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_first_data_bar_message(value).expect("Failed to decode FirstTradeBar")
    }
}
