use crate::prelude::{DataType, MessageType, StartDataMessage};
use common::prelude::{ExchangeID, TimeResolution};

impl StartDataMessage {
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    pub fn client_id(&self) -> &u16 {
        &self.client_id
    }
    pub fn exchange_id(&self) -> &ExchangeID {
        &self.exchange_id
    }
    pub fn symbol_id(&self) -> &u16 {
        &self.symbol_id
    }
    pub fn data_type_id(&self) -> &DataType {
        &self.data_type_id
    }
    pub fn time_resolution(&self) -> &TimeResolution {
        &self.time_resolution
    }
}
