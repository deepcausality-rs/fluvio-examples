use crate::prelude::{DataType, MessageType};
use common::prelude::ExchangeID;
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
    symbol_id: u16,
    data_type_id: DataType,
}

impl StartDataMessage {
    pub fn new(
        client_id: u16,
        exchange_id: ExchangeID,
        symbol_id: u16,
        data_type_id: DataType,
    ) -> Self {
        let message_type = MessageType::StartData;

        Self {
            message_type,
            client_id,
            exchange_id,
            symbol_id,
            data_type_id,
        }
    }
}

impl From<&[u8]> for StartDataMessage {
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_start_data_message(value).expect("Failed to decode start data message")
    }
}
