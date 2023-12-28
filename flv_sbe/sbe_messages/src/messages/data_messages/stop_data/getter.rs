use crate::messages::data_messages::stop_data::StopDataMessage;
use crate::prelude::MessageType;
use common::prelude::{ExchangeID, SymbolID};

impl StopDataMessage {
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    pub fn client_id(&self) -> &u16 {
        &self.client_id
    }
    pub fn exchange_id(&self) -> &ExchangeID {
        &self.exchange_id
    }
    pub fn symbol_id(&self) -> &SymbolID {
        &self.symbol_id
    }
}
