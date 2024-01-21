mod display;
mod getters;
mod sbe_decode;
mod sbe_encode;

use crate::prelude::MessageType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct LastOHLCVBar {
    message_type: MessageType,
    symbol_id: u16,
}

impl LastOHLCVBar {
    /// Creates a new LastOHLCVBar instance.
    ///
    /// Sets the message_type to LastOHLCVBar.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - u16 symbol ID
    ///
    /// # Returns
    ///
    /// LastOHLCVBar instance
    ///
    pub fn new(symbol_id: u16) -> Self {
        let message_type = MessageType::LastOHLCVBar;
        Self {
            message_type,
            symbol_id,
        }
    }
}

impl From<&[u8]> for LastOHLCVBar {
    /// Implements the From trait to decode a LastOHLCVBar from a byte slice.
    ///
    /// Calls the sbe_decode::decode_last_data_bar_message function to decode the message.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte slice to decode
    ///
    /// # Returns
    ///
    /// Decoded LastOHLCVBar
    ///
    /// # Errors
    ///
    /// Panics if decode fails
    ///
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_last_data_bar_message(value)
            .expect("Failed to decode LastDataBar message")
    }
}
