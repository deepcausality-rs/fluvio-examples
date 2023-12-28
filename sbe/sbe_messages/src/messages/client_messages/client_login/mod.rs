use crate::prelude::MessageType;
use serde::{Deserialize, Serialize};

mod display;
mod getters;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ClientLoginMessage {
    message_type: MessageType,
    client_id: u16,
}

impl ClientLoginMessage {
    pub fn new(client_id: u16) -> Self {
        let message_type = MessageType::ClientLogin;

        Self {
            message_type,
            client_id,
        }
    }
}

impl From<&[u8]> for ClientLoginMessage {
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_client_login_message(value).expect("Failed to decode ClientLoginMessage")
    }
}
