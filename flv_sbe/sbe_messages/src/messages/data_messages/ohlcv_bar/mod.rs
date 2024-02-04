use crate::errors::{SbeDecodeError, SbeEncodeError};
use common::prelude::OHLCVBar;
use serde::{Deserialize, Serialize};

pub mod sbe_decoder;
pub mod sbe_encoder;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct SbeOHLCVBar {}

impl SbeOHLCVBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl SbeOHLCVBar {
    /// Encodes an OHLCVBar into an SBE message buffer.
    ///
    /// # Parameters
    ///
    /// - `bar` - The OHLCVBar to encode
    ///
    /// # Returns
    ///
    /// A Result containing:
    ///
    /// - The size of the encoded message
    /// - The encoded message buffer
    pub fn encode(bar: OHLCVBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_encoder::encode_data_bar_message(bar)
    }

    /// Decodes an SBE message buffer into an OHLCVBar.
    ///
    /// # Parameters
    ///
    /// - `buffer` - The SBE encoded message buffer
    ///
    /// # Returns
    ///
    /// A Result containing the decoded OHLCVBar or a decoding error.
    #[inline]
    pub fn decode(buffer: &[u8]) -> Result<OHLCVBar, SbeDecodeError> {
        sbe_decoder::decode_data_bar_message(buffer)
    }
}
