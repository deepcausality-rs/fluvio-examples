use serde::{Deserialize, Serialize};

use common::prelude::ExchangeID;

use crate::prelude::MessageType;

mod display;
mod getter;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct StopAllDataMessage {
    message_type: MessageType,
    client_id: u16,
    exchange_id: ExchangeID,
}

impl StopAllDataMessage {
    pub fn new(client_id: u16, exchange_id: ExchangeID) -> Self {
        let message_type = MessageType::StopAllData;
        Self {
            message_type,
            client_id,
            exchange_id,
        }
    }
}

impl From<&[u8]> for StopAllDataMessage {
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_stop_all_data_message(value)
            .expect("Failed to decode start data message")
    }
}
