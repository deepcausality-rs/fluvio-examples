use crate::prelude::{MessageType, StartDataMessage};

impl StartDataMessage {
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    pub fn client_id(&self) -> &u16 {
        &self.client_id
    }
    pub fn exchange_id(&self) -> &u8 {
        &self.exchange_id
    }
    pub fn symbol_id(&self) -> &u16 {
        &self.symbol_id
    }
}
