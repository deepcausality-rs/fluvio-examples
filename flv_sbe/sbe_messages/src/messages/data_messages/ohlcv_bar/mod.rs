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
    pub fn encode_data_bar_message(bar: OHLCVBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_encoder::encode_data_bar_message(bar)
    }

    pub fn decode_data_bar_message(buffer: &[u8]) -> Result<OHLCVBar, SbeDecodeError> {
        sbe_decoder::decode_data_bar_message(buffer)
    }
}
