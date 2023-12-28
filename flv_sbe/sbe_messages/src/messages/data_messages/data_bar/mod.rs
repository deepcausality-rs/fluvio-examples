use crate::errors::{SbeDecodeError, SbeEncodeError};
use common::prelude::DataBar;
use serde::{Deserialize, Serialize};

pub mod sbe_decoder;
pub mod sbe_encoder;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct SbeDataBar {}

impl SbeDataBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl SbeDataBar {
    pub fn encode_data_bar_message(bar: DataBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_encoder::encode_data_bar_message(bar)
    }

    pub fn decode_data_bar_message(buffer: &[u8]) -> Result<DataBar, SbeDecodeError> {
        sbe_decoder::decode_data_bar_message(buffer)
    }
}
