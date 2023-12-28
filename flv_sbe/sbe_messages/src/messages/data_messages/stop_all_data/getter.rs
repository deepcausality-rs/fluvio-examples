use crate::messages::data_messages::stop_all_data::StopAllDataMessage;
use crate::prelude::MessageType;
use common::prelude::ExchangeID;

impl StopAllDataMessage {
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    pub fn client_id(&self) -> &u16 {
        &self.client_id
    }
    pub fn exchange_id(&self) -> &ExchangeID {
        &self.exchange_id
    }
}
