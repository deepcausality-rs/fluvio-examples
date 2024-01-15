use crate::prelude::{FirstOHLCVBar, MessageType};

impl FirstOHLCVBar {
    pub fn message_type(&self) -> MessageType {
        self.message_type
    }
    pub fn symbol_id(&self) -> u16 {
        self.symbol_id
    }
}
