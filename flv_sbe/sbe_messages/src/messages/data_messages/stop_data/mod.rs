use serde::{Deserialize, Serialize};

use common::prelude::ExchangeID;

use crate::prelude::{DataType, MessageType};

mod display;
mod getter;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct StopDataMessage {
    message_type: MessageType,
    client_id: u16,
    exchange_id: ExchangeID,
    symbol_id: u16,
    data_type_id: DataType,
}

impl StopDataMessage {
    pub fn new(
        client_id: u16,
        exchange_id: ExchangeID,
        symbol_id: u16,
        data_type_id: DataType,

    ) -> Self {
        let message_type = MessageType::StopData;
        Self {
            message_type,
            client_id,
            exchange_id,
            symbol_id,
            data_type_id,
        }
    }
}

impl From<&[u8]> for StopDataMessage {
    #[inline]
    fn from(buffer: &[u8]) -> Self {
        sbe_decode::decode_stop_data_message(buffer).expect("Failed to decode start data message")
    }
}
