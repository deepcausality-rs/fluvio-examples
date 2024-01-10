use crate::prelude::MessageType;
use sbe_bindings::ClientErrorType;
use serde::{Deserialize, Serialize};

mod getters;
mod sbe_decode;
mod sbe_encode;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ClientErrorMessage {
    message_type: MessageType,
    client_id: u16,
    client_error_type: ClientErrorType,
}

impl ClientErrorMessage {
    pub fn new(client_id: u16, client_error_type: ClientErrorType) -> Self {
        let message_type = MessageType::ClientError;
        Self {
            message_type,
            client_id,
            client_error_type,
        }
    }
}
impl From<&[u8]> for ClientErrorMessage {
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_client_error_message(value).expect("Failed to decode ClientLoginMessage")
    }
}
