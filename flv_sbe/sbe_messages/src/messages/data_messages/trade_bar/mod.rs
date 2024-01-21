mod sbe_decode;
mod sbe_encode;

use crate::errors::{SbeDecodeError, SbeEncodeError};
use common::prelude::TradeBar;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct SbeTradeBar {}

impl SbeTradeBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl SbeTradeBar {
    /// Encodes a TradeBar message to a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `bar` - TradeBar to encode
    ///
    /// # Returns
    ///
    /// (usize, `Vec<u8>`) - Tuple containing encoded size and byte buffer
    ///
    /// # Errors
    ///
    /// Returns Err if encoding fails
    ///
    /// # Remarks
    ///
    /// Calls sbe_encode::encode_trade_bar_message to perform encoding
    ///
    pub fn encode(bar: TradeBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_encode::encode_trade_bar_message(bar)
    }

    /// Decodes a TradeBar message from a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Byte buffer containing encoded TradeBar message
    ///
    /// # Returns
    ///
    /// Decoded TradeBar on success
    ///
    /// # Errors
    ///
    /// Returns Err if decoding fails
    ///
    /// # Remarks
    ///
    /// Calls sbe_decode::decode_trade_bar_message to perform decoding
    ///
    pub fn decode(buffer: &[u8]) -> Result<TradeBar, SbeDecodeError> {
        sbe_decode::decode_trade_bar_message(buffer)
    }
}
