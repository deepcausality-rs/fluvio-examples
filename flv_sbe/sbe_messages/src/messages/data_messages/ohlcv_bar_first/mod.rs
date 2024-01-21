mod display;
mod getters;
mod sbe_decode;
mod sbe_encode;

use crate::prelude::MessageType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FirstOHLCVBar {
    message_type: MessageType,
    symbol_id: u16,
}

impl FirstOHLCVBar {
    /// Creates a new FirstOHLCVBar instance.
    ///
    /// Sets the message_type to FirstOHLCVBar.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - u16 symbol ID
    ///
    /// # Returns
    ///
    /// FirstOHLCVBar instance
    ///
    pub fn new(symbol_id: u16) -> Self {
        let message_type = MessageType::FirstOHLCVBar;
        Self {
            message_type,
            symbol_id,
        }
    }
}

impl From<&[u8]> for FirstOHLCVBar {
    /// Implements the From trait to decode a FirstOHLCVBar from a byte slice.
    ///
    /// Calls the sbe_decode::decode_first_data_bar_message function to decode the message.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte slice to decode
    ///
    /// # Returns
    ///
    /// Decoded FirstOHLCVBar
    ///
    /// # Errors
    ///
    /// Panics if decode fails
    ///
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_first_data_bar_message(value)
            .expect("Failed to decode FirstDataBar message")
    }
}
