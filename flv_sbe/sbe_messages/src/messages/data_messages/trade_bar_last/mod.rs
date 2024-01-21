use crate::prelude::MessageType;
use serde::{Deserialize, Serialize};

mod display;
mod getters;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct LastTradeBar {
    message_type: MessageType,
    symbol_id: u16,
}

impl LastTradeBar {
    /// Creates a new LastTradeBar instance.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - Symbol ID for the bar
    ///
    /// # Returns
    ///
    /// New LastTradeBar instance
    ///
    /// # Remarks
    ///
    /// Sets message_type to LastTradeBar
    ///
    pub fn new(symbol_id: u16) -> Self {
        let message_type = MessageType::LastTradeBar;
        Self {
            message_type,
            symbol_id,
        }
    }
}

impl From<&[u8]> for LastTradeBar {
    /// Decodes a LastTradeBar message from a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte buffer containing encoded LastTradeBar message
    ///
    /// # Returns
    ///
    /// Decoded LastTradeBar on success
    ///
    /// # Errors
    ///
    /// Returns Err if decoding fails
    ///
    /// # Remarks
    ///
    /// Calls sbe_decode::decode_last_trade_bar_message to decode message
    ///
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_last_trade_bar_message(value)
            .expect("Failed to decode LastTradeBar message")
    }
}
