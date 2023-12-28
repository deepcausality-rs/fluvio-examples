use crate::prelude::{ClientLogoutMessage, MessageType};

impl ClientLogoutMessage {
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    pub fn client_id(&self) -> u16 {
        self.client_id
    }
}
