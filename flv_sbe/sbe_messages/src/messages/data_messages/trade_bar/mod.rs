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
    pub fn encode_data_bar_message(bar: TradeBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_encode::encode_data_bar_message(bar)
    }

    pub fn decode_trade_bar_message(buffer: &[u8]) -> Result<TradeBar, SbeDecodeError> {
        sbe_decode::decode_trade_bar_message(buffer)
    }
}
