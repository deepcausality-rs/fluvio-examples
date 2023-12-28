use crate::prelude::MessageType;
use common::prelude::{ExchangeID, SymbolID};
use serde::{Deserialize, Serialize};

mod display;
mod getter;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct StartDataMessage {
    message_type: MessageType,
    client_id: u16,
    exchange_id: ExchangeID,
    symbol_id: SymbolID,
}

impl StartDataMessage {
    pub fn new(client_id: u16, exchange_id: ExchangeID, symbol_id: SymbolID) -> Self {
        let message_type = MessageType::StartData;
        Self {
            message_type,
            client_id,
            exchange_id,
            symbol_id,
        }
    }
}

impl From<&[u8]> for StartDataMessage {
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_start_data_message(value).expect("Failed to decode start data message")
    }
}
