mod sbe_decode;
mod sbe_encode;
mod getters;
mod display;

use crate::prelude::MessageType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct LastDataBar {
    message_type: MessageType,
    symbol_id: u16,
}

impl LastDataBar {
    pub fn new(symbol_id: u16) -> Self {
        let message_type = MessageType::LastDataBar;
        Self {
            message_type,
            symbol_id,
        }
    }
}

impl From<&[u8]> for LastDataBar {
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_last_data_bar_message(value)
            .expect("Failed to decode LastDataBar message")
    }
}
