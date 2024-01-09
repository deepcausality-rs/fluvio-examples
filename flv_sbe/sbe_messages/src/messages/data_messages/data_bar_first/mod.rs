mod sbe_decode;
mod sbe_encode;
use crate::prelude::MessageType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FirstDataBar {
    message_type: MessageType,
    symbol_id: u16,
}

impl FirstDataBar {
    pub fn new(symbol_id: u16) -> Self {
        let message_type = MessageType::FirstDataBar;

        Self {
            message_type,
            symbol_id,
        }
    }
}

impl FirstDataBar {
    pub fn message_type(&self) -> MessageType {
        self.message_type
    }
    pub fn symbol_id(&self) -> u16 {
        self.symbol_id
    }
}

impl fmt::Display for FirstDataBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FirstDataBar {{ message_type: {:?} }}",
            self.message_type
        )
    }
}
