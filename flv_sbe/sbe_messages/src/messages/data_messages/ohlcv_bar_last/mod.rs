mod display;
mod getters;
mod sbe_decode;
mod sbe_encode;

use crate::prelude::MessageType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct LastOHCLVBar {
    message_type: MessageType,
    symbol_id: u16,
}

impl LastOHCLVBar {
    pub fn new(symbol_id: u16) -> Self {
        let message_type = MessageType::LastOHLCBar;
        Self {
            message_type,
            symbol_id,
        }
    }
}

impl From<&[u8]> for LastOHCLVBar {
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_last_data_bar_message(value)
            .expect("Failed to decode LastDataBar message")
    }
}
