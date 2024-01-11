use serde::{Deserialize, Serialize};
use crate::prelude::MessageType;

mod getters;
mod display;
mod sbe_encode;
mod sbe_decode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct LastTradeBar {
    message_type: MessageType,
    symbol_id: u16,
}

impl LastTradeBar {
    pub fn new(symbol_id: u16) -> Self {
        let message_type = MessageType::LastTradeBar;
        Self {
            message_type,
            symbol_id,
        }
    }
}

impl From<&[u8]> for  LastTradeBar{
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_last_trade_bar_message(value)
            .expect("Failed to decode LastTradeBar message")

    }
}