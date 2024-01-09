use sbe_bindings::ClientErrorType;
use crate::prelude::{ClientErrorMessage, MessageType};

impl ClientErrorMessage {
    pub fn message_type(&self) -> MessageType {
        self.message_type
    }
    pub fn client_id(&self) -> u16 {
        self.client_id
    }
    pub fn client_error_type(&self) -> ClientErrorType {
        self.client_error_type
    }
}
