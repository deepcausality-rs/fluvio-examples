use crate::prelude::{LastOHLCVBar, MessageType};

impl LastOHLCVBar {
    pub fn message_type(&self) -> MessageType {
        self.message_type
    }
    pub fn symbol_id(&self) -> u16 {
        self.symbol_id
    }
}
