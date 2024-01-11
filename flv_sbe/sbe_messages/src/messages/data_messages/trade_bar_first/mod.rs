use serde::{Deserialize, Serialize};
use crate::prelude::MessageType;

mod getters;
mod display;
mod sbe_encode;
mod sbe_decode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FirstTradeBar {
    message_type: MessageType,
    symbol_id: u16,
}

impl FirstTradeBar {
    pub fn new(symbol_id: u16) -> Self {
        let message_type = MessageType::FirstTradeBar;
        Self {
            message_type,
            symbol_id,
        }
    }
}

impl From<&[u8]> for FirstTradeBar {
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_first_data_bar_message(value)
            .expect("Failed to decode FirstTradeBar")
    }
}